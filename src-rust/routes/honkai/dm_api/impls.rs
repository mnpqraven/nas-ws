use std::sync::Arc;

use regex::{Captures, Regex};
use tracing::{instrument, info};

use super::BigTraceInfo;

impl BigTraceInfo {
    const DESC_IDENT: &str = r"#\d\[.\d?\]%?";
    #[allow(dead_code)]
    #[instrument(ret)]
    pub fn parse_description(&self) -> Vec<String> {
        // desc
        // "Deals Lightning DMG equal to #1[i]% of Kafka's ATK to a single enemy.",
        // params
        // [ [0.5], [0.6] ,.. , [] ]
        let regex = Regex::new(Self::DESC_IDENT).unwrap();
        let mut res: Vec<String> = vec![];
        for param in self.params.iter() {
            let result = regex.replace_all(&self.desc, |caps: &Captures| {
                let mut res = String::new();
                for cap in caps.iter().flatten() {
                    let is_percent: bool = cap.as_str().ends_with('%');

                    // let index = cap.as_str().chars().nth(1).unwrap().to_digit(10).unwrap() as usize;

                    let params_data = match is_percent {
                        true => param * 100.0,
                        false => *param,
                    };
                    match is_percent {
                        true => res.push_str(&format!("{:.2}%", &params_data)),
                        false => res.push_str(&format!("{:.2}", &params_data)),
                    }
                }
                res
            });
            res.push(result.to_string());
        }
        info!("{:?}", res);
        res
    }

    pub fn split_description(&self) -> Arc<[Arc<str>]> {
        let regex = Regex::new(Self::DESC_IDENT).unwrap();
        let t: Arc<[Arc<str>]> = regex.split(&self.desc).map(|e| e.into()).collect();
        t
    }

    /// returns a tuple of
    /// 1. index of the params value
    /// 2. whether the params value should be displayed as percentage
    pub fn get_sorted_params_inds(&self) -> Vec<(usize, bool)> {
        let regex = Regex::new(Self::DESC_IDENT).unwrap();
        let inds = regex
            .find_iter(&self.desc)
            .map(|e| {
                let ind: usize = (e.as_str().chars().nth(1).unwrap().to_digit(10).unwrap() - 1)
                    .try_into()
                    .unwrap();
                let is_percent = e.as_str().ends_with('%');
                (ind, is_percent)
            })
            .collect::<Vec<(usize, bool)>>();
        inds
    }
}
