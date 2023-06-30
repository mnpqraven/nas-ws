use clap::Parser;
use nas_ws::routes::{
    honkai::{
        mhy_api::{
            internal::categorizing::{DbCharacter, DbCharacterEidolon, DbCharacterSkillTree},
            types_parsed::{shared::DbAttributeProperty, MihoResponse},
        },
        patch::types::{Patch, PatchBanner}, dm_api::BigTraceInfo,
    },
    utils::{mock_hsr_log::Log, mock_hsr_stat::MvpWrapper},
};
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
        Schema::new(schema_for!(MihoResponse), "MihoResponse"),
        // Schema::new(schema_for!(EstimateCfg), "EstimateCfg"),
        Schema::new(schema_for!(MvpWrapper), "MvpWrapper"),
        Schema::new(schema_for!(Log), "Log"),
        Schema::new(schema_for!(PatchBanner), "PatchBanner"),
        Schema::new(schema_for!(DbCharacter), "DbCharacter"),
        Schema::new(schema_for!(DbCharacterSkillTree), "DbCharacterSkillTree"),
        Schema::new(schema_for!(DbCharacterEidolon), "DbCharacterEidolon"),
        Schema::new(schema_for!(DbAttributeProperty), "DbAttributeProperty"),
        Schema::new(schema_for!(BigTraceInfo), "BigTraceInfo"),
        Schema::new(schema_for!(Patch), "Patch"),
    ];

    for Schema { root, name } in type_names.into_iter() {
        let pretty_data = serde_json::to_string_pretty(&root)?;
        fs::write(schema_path.join(format!("{name}.json")), pretty_data)?;
        println!("Type {name} generated");
    }
    Ok(())
}
