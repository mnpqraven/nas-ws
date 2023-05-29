use serde::Deserialize;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Pull {
    pub draw_number: u32,
    pub rate: f32,
}

#[derive(Debug)]
pub struct SurveyRate(pub Vec<Pull>);

impl Default for SurveyRate {
    fn default() -> Self {
        let mut gacha_rate = SurveyRate(vec![]);
        let path = match true {
            true if Path::new("../assets/rate.csv").exists() => "../assets/rate.csv",
            true if Path::new("assets/rate.csv").exists() => "assets/rate.csv",
            _ => {
                panic!("assets not found, run `cargo run --bin tasks`");
            }
        };
        match csv::Reader::from_path(path) {
            Ok(mut rdr) => {
                for pull in rdr.deserialize::<Pull>().flatten() {
                    gacha_rate.0.push(pull);
                }
            }
            Err(_) => {
                panic!("assets not found, run `cargo run --bin tasks`");
            }
        }
        gacha_rate
    }
}
