// hutao bot's calc transpiled to rust
// https://gist.github.com/Tibowl/7ae7395e000843ad4882030b9c4703b5

use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::{extract::rejection::JsonRejection, Json};
use response_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::error;
use vercel_runtime::{Body, Response, StatusCode};

struct Banner {
    banner_name: String,
    // rate of the featured ssr (0.5 for character, 0.75 for LC)
    banner: f64,
    guaranteed: f64,
    // not yet implemented, genshin epitomized path ???
    guaranteed_pity: Option<i32>,
    min_const: i32,
    max_const: i32,
    // pity count (90 for char, 80 lc)
    max_pity: i32,
    // constFormat: string
    // constName: string
    rate: Box<dyn Fn(i32) -> f64>, // (pity: number) => number
}

#[derive(Debug, Clone)]
struct Sim {
    eidolon: i32,
    rate: f64,
    pity: i32,
    guaranteed: bool,
    guaranteed_pity: i32,
}
#[derive(Debug, Serialize, JsonResponse, Clone)]
pub struct ReducedSim {
    pub eidolon: i32,
    pub rate: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProbabilityRatePayload {
    pub current_eidolon: i32,
    pub pity: i32,
    pub pulls: i32,
    pub next_guaranteed: bool,
    pub enpitomized_pity: Option<i32>,
    pub banner: BannerType,
}

#[derive(Debug, Deserialize)]
pub enum BannerType {
    SSR,
    SR,
    LC,
}
// master struct
#[derive(Debug, Serialize, JsonResponse, Clone)]
pub struct ProbabilityRateResponse {
    pub roll_budget: i32,
    pub data: Vec<Vec<ReducedSim>>,
}

pub async fn probability_rate(
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
        BannerType::SSR => Banner::char_ssr(),
        BannerType::SR => Banner::char_sr(),
        BannerType::LC => Banner::weapon(),
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
        data: calcs,
    };

    Ok(Json(master_prob_rate))
}

fn pity_rate(base_rate: f64, pity_start: i32) -> Box<dyn Fn(i32) -> f64> {
    let func = move |pity: i32| match pity < pity_start {
        true => base_rate,
        false => base_rate + base_rate * 10.0 * (pity - pity_start + 1) as f64,
    };
    Box::new(func)
}

impl Banner {
    fn char_ssr() -> Self {
        Self {
            banner_name: "5* Banner character".into(),
            banner: 0.5,
            guaranteed: 1.0,
            guaranteed_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 90,
            rate: pity_rate(0.6, 74),
        }
    }
    fn char_sr() -> Self {
        Self {
            banner_name: "Specific 4* banner character".into(),
            banner: 0.5,
            guaranteed: 0.333333333,
            guaranteed_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 10,
            rate: pity_rate(5.1, 9),
        }
    }
    fn weapon() -> Self {
        Self {
            banner_name: "Specific 5* banner weapon".into(),
            banner: 0.75,
            guaranteed: 0.5,
            guaranteed_pity: Some(3),
            min_const: 0,
            max_const: 5,
            max_pity: 80,
            rate: pity_rate(0.7, 63),
        }
    }
}

fn calc_sims_regular(
    current_eidolon: i32,
    pity: i32,
    pulls: i32,
    guaranteed: bool,
    guaranteed_pity: i32,
    banner: Banner,
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

fn calc_sims_int(starter_sim: Sim, pulls: i32, banner: Banner) -> Vec<Vec<ReducedSim>> {
    let mut smal_sims = vec![starter_sim];
    let mut sims = calc_sims_exact(&mut smal_sims, pulls, &banner);

    sims.iter_mut()
        .map(|e| {
            let mut reduced_sim: HashMap<i32, ReducedSim> = HashMap::new();
            e.iter().for_each(|inner_sim| {
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
        })
        .collect()
}

// TODO: doesn't work anymore, refactor using foreach closure above
fn sim_to_reduced(mut sim: &mut Vec<Sim>) -> Vec<ReducedSim> {
    let mut res: Vec<ReducedSim> = vec![];
    for sim in sim.iter_mut() {
        if sim.rate != 0.0 {
            let mut other = res.get_mut((sim.eidolon + 1) as usize);
            if let Some(t) = other {
                t.rate += sim.rate
            } else {
                let mut new_reduced = ReducedSim {
                    eidolon: sim.eidolon,
                    rate: sim.rate,
                };
                other = Some(&mut new_reduced)
            }
        }
    }
    res
}

fn calc_sims_exact(sims: &mut Vec<Sim>, pulls: i32, banner: &Banner) -> Vec<Vec<Sim>> {
    let mut all_sims: Vec<Vec<Sim>> = vec![sims.clone()];
    for _ in 0..pulls {
        let mut new_sims: HashMap<i32, Sim> = HashMap::new();

        let mut add_or_merge = |sim: &Sim| {
            if sim.rate > 0.0 {
                let guaranteed = match sim.guaranteed {
                    true => 1,
                    false => 0,
                };
                let v = sim.pity
                    + (banner.max_pity + 1)
                        * ((sim.eidolon + 1)
                            + ((banner.max_const + 2) * (guaranteed + (2 * sim.guaranteed_pity))));

                let other = new_sims.get_mut(&v);

                if let Some(existing_sim) = other {
                    existing_sim.rate += sim.rate;
                } else {
                    new_sims.insert(v, sim.clone());
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
        // Object.values(newSims);
        let to_append: Vec<Sim> = new_sims.into_iter().map(|e| e.1).collect();
        *sims = to_append.clone();
        all_sims.push(to_append);
    }
    all_sims
}
// debug
#[cfg(test)]
mod test {
    use super::{calc_sims_regular, Banner};

    #[test]
    fn test() {
        let calcs = calc_sims_regular(-1, 0, 5, false, 0, Banner::char_ssr());
        dbg!(calcs);
    }
}
