use crate::routes::honkai::mhy_api::types_parsed::shared::Property;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TextMap(pub HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema)]
pub struct Param {
    #[serde(alias = "Value")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LightConeRarity {
    CombatPowerLightconeRarity3 = 3,
    CombatPowerLightconeRarity4 = 4,
    CombatPowerLightconeRarity5 = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbilityProperty {
    #[serde(alias = "PropertyType")]
    property_type: Property,
    #[serde(alias = "Value")]
    value: Param,
}

impl From<Param> for f64 {
    fn from(val: Param) -> Self {
        val.value
    }
}
