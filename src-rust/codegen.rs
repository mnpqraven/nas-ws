use nas_ws::routes::honkai::{
    jade_estimate::types::EstimateCfg,
    mhy_api::{internal::categorizing::{DbCharacterSkillTree, DbCharacterEidolon, DbCharacter}, types_parsed::{MihoResponse, shared::DbAttributeProperty}},
    patch::types::{Patch, PatchBanner},
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

fn main() -> Result<(), Box<dyn Error>> {
    // ../vercel/jade-tracker-vercel/src/bindings
    let schema_path = Path::new("../vercel/jade-tracker-vercel/.schemas/");

    fs::remove_dir_all(schema_path).unwrap();
    // create dir if doesn't exist
    fs::create_dir_all(schema_path).unwrap();

    let type_names = vec![
        Schema::new(schema_for!(MihoResponse), "MihoResponse"),
        Schema::new(schema_for!(EstimateCfg), "EstimateCfg"),
        Schema::new(schema_for!(PatchBanner), "PatchBanner"),
        Schema::new(schema_for!(DbCharacter), "DbCharacter"),
        Schema::new(schema_for!(DbCharacterSkillTree), "DbCharacterSkillTree"),
        Schema::new(schema_for!(DbCharacterEidolon), "DbCharacterEidolon"),
        Schema::new(schema_for!(DbAttributeProperty), "DbAttributeProperty"),
        Schema::new(schema_for!(Patch), "Patch"),
    ];

    for Schema { root, name } in type_names.into_iter() {
        let pretty_data = serde_json::to_string_pretty(&root)?;
        fs::write(schema_path.join(format!("{name}.json")), pretty_data)?;
        println!("Type {name} generated");
    }
    Ok(())
}
