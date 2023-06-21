#[allow(dead_code)]
pub const RELIC_SET_DICT: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/relic_sets.json";

#[allow(dead_code)]
pub const RELIC_PIECES_DICT: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/relics.json";

#[cfg(target_os = "windows")]
pub const CHARACTER_LOCAL: &str = "c:\\tmp\\characters.json";
#[cfg(target_os = "linux")]
pub const CHARACTER_LOCAL: &str = "/tmp/characters.json";

pub const CHARACTER_REMOTE: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/characters.json";

#[cfg(target_os = "windows")]
pub const CHARACTER_SKILL_LOCAL: &str = "c:\\tmp\\character_skills.json";
#[cfg(target_os = "linux")]
pub const CHARACTER_SKILL_LOCAL: &str = "/tmp/character_skills.json";
pub const CHARACTER_SKILL_REMOTE: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/character_skills.json";

#[cfg(target_os = "windows")]
pub const CHARACTER_SKILL_TREE_LOCAL: &str = "c:\\tmp\\character_skill_trees.json";
#[cfg(target_os = "linux")]
pub const CHARACTER_SKILL_TREE_LOCAL: &str = "/tmp/character_skill_trees.json";
pub const CHARACTER_SKILL_TREE_REMOTE: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/character_skill_trees.json";

#[cfg(target_os = "windows")]
pub const CHARACTER_EIDOLON_LOCAL: &str = "c:\\tmp\\character_eidolons.json";
#[cfg(target_os = "linux")]
pub const CHARACTER_EIDOLON_LOCAL: &str = "/tmp/character_eidolons.json";
pub const CHARACTER_EIDOLON_REMOTE: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/character_ranks.json";

#[cfg(target_os = "windows")]
pub const ATTRIBUTE_PROPERTY_LOCAL: &str = "c:\\tmp\\attribute_properties.json";
#[cfg(target_os = "linux")]
pub const ATTRIBUTE_PROPERTY_LOCAL: &str = "/tmp/attribute_properties.json";
pub const ATTRIBUTE_PROPERTY_REMOTE: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/properties.json";