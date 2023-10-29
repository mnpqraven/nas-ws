use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg(test)]
mod tests;

const DESC_SANITIZE: &str = r"(?<codeleft><.+?>)+(?<coderight><.+?>)*";
const DESC_IDENT: &str = r"(?<codeleft><.+?>)*(?<slot>#\d\[.\d?\]%?)(?<coderight><.+?>)*";

// Usually served to the front end and connect string slices there
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ParameterizedDescription(pub Vec<String>);

impl ParameterizedDescription {
    pub fn values(&self) -> Vec<String> {
        self.0.clone()
    }
}

/// a tuple of
/// 1. index of the params value
/// 2. whether the params value should be displayed as percentage
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ParameterValue(pub (f64, bool));

impl From<String> for ParameterizedDescription {
    fn from(value: String) -> Self {
        // sanitize the string, removing all xml tags
        let sanitizer = Regex::new(DESC_SANITIZE).unwrap();
        let value = sanitizer.replace_all(&value, "");

        let regex = Regex::new(DESC_IDENT).unwrap();

        let res: Vec<String> = regex.split(&value).map(|e| e.into()).collect();
        Self(res)
    }
}

/// A tuple of
/// 1. index of the params value
/// 2. whether the params value should be displayed as percentage
fn get_sorted_params_id(desc: &str) -> Vec<(usize, bool)> {
    let regex = Regex::new(DESC_IDENT).unwrap();

    let inds = regex
        .captures_iter(desc)
        .map(|e| {
            let ind: usize = (e
                .name("slot")
                .unwrap()
                .as_str()
                .chars()
                .nth(1)
                .unwrap()
                .to_digit(10)
                .unwrap()
                - 1)
            .try_into()
            .unwrap();
            let is_percent = e.name("slot").unwrap().as_str().ends_with('%');
            (ind, is_percent)
        })
        .collect::<Vec<(usize, bool)>>();
    inds
}

/// THIS HAS A DEPTH OF 1
///
/// for current application it means sorting a param list at specific skill
/// level, if taken from json data
/// returns sorted list and a flag noting this value is a percent or not
pub fn get_sorted_params(list: Vec<f64>, desc: &str) -> Vec<ParameterValue> {
    // get index from a and b
    // do normal sort rule but for our tuple vec using a,b index
    let mut params: Vec<ParameterValue> = vec![];
    let sorter = get_sorted_params_id(desc);
    for (key, is_percent) in sorter.into_iter() {
        params.push(ParameterValue((*list.get(key).unwrap(), is_percent)));
    }
    params
}

impl Display for ParameterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_percent = self.0 .1;
        let fmt = match is_percent {
            true => format!("{:.2} %", self.0 .0 * 100.0),
            false => format!("{:.0}", self.0 .0),
        };
        write!(f, "{}", fmt)
    }
}
