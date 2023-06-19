#[allow(dead_code)]
pub const RELIC_SET_DICT: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/relic_sets.json";

#[allow(dead_code)]
pub const RELIC_PIECES_DICT: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/relics.json";

pub const CHARACTER_DICT: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/characters.json";

#[cfg(target_os = "windows")]
pub const CHARACTER_SKILL_LOCAL: &str = "../dump_data/character_skills.json";
#[cfg(target_os = "linux")]
pub const CHARACTER_SKILL_LOCAL: &str = "/tmp/character_skills.json";
pub const CHARACTER_SKILL_REMOTE: &str =
    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_new/en/character_skills.json";
