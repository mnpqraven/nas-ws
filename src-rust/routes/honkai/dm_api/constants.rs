#[cfg(target_os = "windows")]
pub const TEXT_MAP_LOCAL: &str = "c:\\tmp\\text_map.json";
#[cfg(target_os = "linux")]
pub const TEXT_MAP_LOCAL: &str = "/tmp/text_map.json";

pub const TEXT_MAP_REMOTE: &str =
    "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/TextMap/TextMapEN.json";
