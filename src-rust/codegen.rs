use clap::Parser;
use nas_ws::routes::{honkai::{
    dm_api::{
        atlas::SignatureAtlas,
        character::{
            eidolon::AvatarRankConfig, promotion_config::AvatarPromotionConfig,
            types::AvatarConfig,
        },
        character_skill::types::AvatarSkillConfig,
        equipment::{
            equipment_config::EquipmentConfig,
            equipment_promotion_config::EquipmentPromotionConfig,
            equipment_skill_config::EquipmentSkillConfig, stat_ranking::EquipmentRanking,
        },
        equipment_skill::skill_tree_config::SkillTreeConfig,
        property::config::AvatarPropertyConfig,
        relic::{set_config::RelicSetConfig, set_skill_config::RelicSetSkillConfig, sub_affix::RelicSubAffixConfig, main_affix::RelicMainAffixConfig, config::RelicConfig},
    },
    patch::types::{Patch, PatchBanner}, banner::types::Banner,
}, utils::mock_hsr_stat::MvpAnalysis};
use schemars::{schema::RootSchema, schema_for};
use std::{error::Error, fs, path::Path};

struct Schema {
    root: RootSchema,
    name: String,
}
impl Schema {
    fn new(schema: RootSchema, name: impl Into<String>) -> Self {
        Self {
            root: schema,
            name: name.into(),
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let schema_path = Path::new(&args.path);

    if schema_path.exists() {
        fs::remove_dir_all(schema_path).unwrap();
    }
    // create dir if doesn't exist
    fs::create_dir_all(schema_path).unwrap();

    let type_names = vec![
        Schema::new(schema_for!(PatchBanner), "PatchBanner"),
        Schema::new(schema_for!(Patch), "Patch"),
        Schema::new(schema_for!(Banner), "Banner"),
        Schema::new(schema_for!(SkillTreeConfig), "SkillTreeConfig"),
        Schema::new(schema_for!(SignatureAtlas), "SignatureAtlas"),
        Schema::new(schema_for!(AvatarConfig), "AvatarConfig"),
        Schema::new(schema_for!(AvatarPromotionConfig), "AvatarPromotionConfig"),
        Schema::new(schema_for!(AvatarSkillConfig), "AvatarSkillConfig"),
        Schema::new(schema_for!(AvatarPropertyConfig), "AvatarPropertyConfig"),
        Schema::new(schema_for!(AvatarRankConfig), "AvatarRankConfig"),
        Schema::new(schema_for!(EquipmentConfig), "EquipmentConfig"),
        Schema::new(
            schema_for!(EquipmentPromotionConfig),
            "EquipmentPromotionConfig",
        ),
        Schema::new(schema_for!(EquipmentSkillConfig), "EquipmentSkillConfig"),
        Schema::new(schema_for!(EquipmentRanking), "EquipmentRanking"),
        Schema::new(schema_for!(RelicConfig), "RelicConfig"),
        Schema::new(schema_for!(RelicSetConfig), "RelicSetConfig"),
        Schema::new(schema_for!(RelicSetSkillConfig), "RelicSetSkillConfig"),
        Schema::new(schema_for!(RelicSubAffixConfig), "RelicSubAffixConfig"),
        Schema::new(schema_for!(RelicMainAffixConfig), "RelicMainAffixConfig"),
        Schema::new(schema_for!(MvpAnalysis), "MvpAnalysis"),
    ];

    for Schema { root, name } in type_names.into_iter() {
        let pretty_data = serde_json::to_string_pretty(&root)?;
        fs::write(schema_path.join(format!("{name}.json")), pretty_data)?;
        println!("Type {name} generated");
    }
    Ok(())
}
