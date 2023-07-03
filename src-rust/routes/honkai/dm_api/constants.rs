#[cfg(target_os = "windows")]
pub const TEXT_MAP_LOCAL: &str = "c:\\tmp\\text_map.json";
#[cfg(target_os = "linux")]
pub const TEXT_MAP_LOCAL: &str = "/tmp/text_map.json";

pub const TEXT_MAP_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/TextMap/TextMapEN.json";

#[cfg(target_os = "windows")]
pub const EQUIPMENT_SKILL_CONFIG_LOCAL: &str = "c:\\tmp\\equipment_skill_config.json";
#[cfg(target_os = "linux")]
pub const EQUIPMENT_SKILL_CONFIG_LOCAL: &str = "/tmp/equipment_skill_config.json";

pub const EQUIPMENT_SKILL_CONFIG_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/EquipmentSkillConfig.json";

#[cfg(target_os = "windows")]
pub const EQUIPMENT_CONFIG_LOCAL: &str = "c:\\tmp\\equipment_config.json";
#[cfg(target_os = "linux")]
pub const EQUIPMENT_CONFIG_LOCAL: &str = "/tmp/equipment_config.json";

pub const EQUIPMENT_CONFIG_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/ExcelOutput/EquipmentConfig.json";
