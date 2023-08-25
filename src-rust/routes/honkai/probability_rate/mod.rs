// hutao bot's calc transpiled to rust
// https://gist.github.com/Tibowl/7ae7395e000843ad4882030b9c4703b5

use self::types::{ProbabilityRatePayload, ProbabilityRateResponse, ReducedSim, Sim};
use super::banner::types::{Banner, BannerIternal, BannerType};
use crate::handler::error::WorkerError;
use axum::{extract::rejection::JsonRejection, Json};
use std::collections::HashMap;
use tracing::error;

pub mod rpc;
pub mod types;

pub async fn handle(
    rpayload: Result<Json<ProbabilityRatePayload>, JsonRejection>,
) -> Result<Json<ProbabilityRateResponse>, WorkerError> {
    if rpayload.is_err() {
        let err = rpayload.unwrap_err();
        error!("{}", err.body_text());
        return Err(WorkerError::ParseData(err.body_text()));
    }
    // safe unwrap
    let Json(payload) = rpayload.unwrap();
    let banner = match payload.banner {
        BannerType::Ssr => Banner::char_ssr().to_internal(pity_rate(0.6, 74)),
        BannerType::Sr => Banner::char_sr().to_internal(pity_rate(5.1, 9)),
        BannerType::Lc => Banner::basic_weapon().to_internal(pity_rate(0.7, 63)),
    };

    let calcs = calc_sims_regular(
        payload.current_eidolon,
        payload.pity,
        payload.pulls,
        payload.next_guaranteed,
        // TODO: not hardcode
        0,
        banner,
    );
    let master_prob_rate = ProbabilityRateResponse {
        roll_budget: payload.pulls,
        data: to_accumulated_rates(&calcs),
    };

    Ok(Json(master_prob_rate))
}

fn to_accumulated_rates(data: &[Vec<ReducedSim>]) -> Vec<Vec<ReducedSim>> {
    data.iter()
        .cloned()
        .map(|mut eidolons_by_pull| {
            // appends eidolon pull if there's less than 7 entries
            (0..7).for_each(|eidolon_number| {
                if !eidolons_by_pull
                    .iter_mut()
                    .any(|e| e.eidolon == eidolon_number)
                {
                    eidolons_by_pull.push(ReducedSim {
                        eidolon: eidolon_number,
                        rate: 0.0,
                    })
                }
            });
            // transform separate rate of each eidolon into accumulated rate
            // for lower eidolons
            let cloned_ref = eidolons_by_pull.clone();
            eidolons_by_pull.iter_mut().for_each(|cell| {
                let higher_eid_cells: Vec<&ReducedSim> = cloned_ref
                    .iter()
                    .filter(|e| e.eidolon > cell.eidolon)
                    .collect();
                cell.rate += higher_eid_cells.iter().map(|e| e.rate).sum::<f64>();
                cell.rate = match cell.rate {
                    x if x > 1.0 => 1.0,
                    y if y < 0.0 => 0.0,
                    _ => cell.rate,
                };
            });
            eidolons_by_pull
        })
        .collect()
}

fn pity_rate(base_rate: f64, pity_start: i32) -> Box<dyn Fn(i32) -> f64> {
    let func = move |pity: i32| match pity < pity_start {
        true => base_rate,
        false => base_rate + base_rate * 10.0 * (pity - pity_start + 1) as f64,
    };
    Box::new(func)
}

fn calc_sims_regular(
    current_eidolon: i32,
    pity: i32,
    pulls: i32,
    guaranteed: bool,
    guaranteed_pity: i32,
    banner: BannerIternal,
) -> Vec<Vec<ReducedSim>> {
    calc_sims_int(
        Sim {
            pity,
            guaranteed,
            guaranteed_pity,
            eidolon: current_eidolon,
            rate: 1.0,
        },
        pulls,
        banner,
    )
}

fn calc_sims_int(starter_sim: Sim, pulls: i32, banner: BannerIternal) -> Vec<Vec<ReducedSim>> {
    let mut smal_sims = vec![starter_sim];
    let sims = calc_sims_exact(&mut smal_sims, pulls, &banner);

    sims.iter().map(|e| sim_to_reduced(e)).collect()
}

fn sim_to_reduced(sim: &[Sim]) -> Vec<ReducedSim> {
    let mut reduced_sim: HashMap<i32, ReducedSim> = HashMap::new();
    sim.iter().for_each(|inner_sim| {
        if inner_sim.rate != 0.0 {
            match reduced_sim.get_mut(&(inner_sim.eidolon + 1)) {
                Some(e) => {
                    e.rate += inner_sim.rate;
                }
                None => {
                    reduced_sim.insert(
                        inner_sim.eidolon + 1,
                        ReducedSim {
                            eidolon: inner_sim.eidolon,
                            rate: inner_sim.rate,
                        },
                    );
                }
            }
        }
    });
    reduced_sim.values().cloned().collect::<Vec<ReducedSim>>()
}

fn calc_sims_exact(sims: &mut Vec<Sim>, pulls: i32, banner: &BannerIternal) -> Vec<Vec<Sim>> {
    let mut all_sims: Vec<Vec<Sim>> = vec![sims.clone()];
    for _ in 0..pulls {
        let mut new_sims: HashMap<i32, Sim> = HashMap::new();

        let mut add_or_merge = |sim: &Sim| {
            if sim.rate > 0.0 {
                let key = sim.pity
                    + (banner.max_pity + 1)
                        * ((sim.eidolon + 1)
                            + ((banner.max_const + 2)
                                * (sim.guaranteed as i32 + (2 * sim.guaranteed_pity))));

                if let Some(existing_sim) = new_sims.get_mut(&key) {
                    existing_sim.rate += sim.rate; // merge
                } else {
                    new_sims.insert(key, sim.clone()); // add
                }
            }
        };

        for sim in sims.iter() {
            if sim.rate <= 0.0 {
                continue;
            }
            if sim.eidolon >= banner.max_const {
                // Limited to C6
                add_or_merge(sim);
                continue;
            }
            let current_pity = sim.pity + 1;

            let mut rate = (banner.rate)(current_pity) / 100.0;
            if rate > 1.0 {
                rate = 1.0;
            } else if rate < 0.0 {
                rate = 0.0;
            }
            let banner_rate: f64 = match banner.guaranteed_pity {
                Some(x) if sim.guaranteed_pity >= x - 1 => 1.0,
                None if sim.guaranteed => 1.0,
                _ => banner.banner,
            };

            // Failed
            if rate < 1.0 {
                let sim = Sim {
                    pity: current_pity,
                    guaranteed: sim.guaranteed,
                    guaranteed_pity: sim.guaranteed_pity,
                    eidolon: sim.eidolon,
                    rate: sim.rate * (1.0 - rate),
                };
                add_or_merge(&sim);
            }

            // Got wanted banner item
            let wanted = Sim {
                pity: 0,
                guaranteed: false,
                guaranteed_pity: 0,
                eidolon: sim.eidolon + 1,
                rate: sim.rate * rate * banner_rate * banner.guaranteed,
            };
            add_or_merge(&wanted);

            // Got banner item but not wanted (eg. wrong rate up 4* char/5* char)
            if banner.guaranteed < 1.0 {
                if banner.guaranteed_pity.is_some()
                    && sim.guaranteed_pity >= banner.guaranteed_pity.unwrap() - 1
                {
                    // epitomized path
                    // https://www.hoyolab.com/article/533196
                    let not_wanted = Sim {
                        pity: 0,
                        guaranteed: false,
                        guaranteed_pity: 0,
                        eidolon: sim.eidolon + 1,
                        rate: sim.rate * rate * banner_rate * (1.0 - banner.guaranteed),
                    };
                    add_or_merge(&not_wanted);
                } else {
                    let guaranteed_pity = match banner.guaranteed_pity {
                        Some(_) => sim.guaranteed_pity + 1,
                        None => 0,
                    };
                    let sim = Sim {
                        pity: 0,
                        guaranteed: false,
                        guaranteed_pity,
                        eidolon: sim.eidolon,
                        rate: sim.rate * rate * banner_rate * (1.0 - banner.guaranteed),
                    };
                    add_or_merge(&sim);
                }
            }

            // Failed banner items (eg. 4* char rate ups vs regular 4*)
            if banner_rate < 1.0 {
                let guaranteed_pity = match banner.guaranteed_pity {
                    Some(_) => sim.guaranteed_pity + 1,
                    None => 0,
                };
                let sim = Sim {
                    pity: 0,
                    guaranteed: true,
                    guaranteed_pity,
                    eidolon: sim.eidolon,
                    rate: sim.rate * rate * (1.0 - banner_rate),
                };
                add_or_merge(&sim)
            }
        }
        let to_append: Vec<Sim> = new_sims.into_iter().map(|e| e.1).collect();
        *sims = to_append.clone();
        all_sims.push(to_append);
    }
    all_sims
}
