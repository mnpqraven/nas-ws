use nas_ws::routes::honkai::{mhy_api::types::MihoResponse, jade_estimate::types::EstimateCfg};
use schemars::schema_for;

fn main() {
    // ../vercel/jade-tracker-vercel/src/bindings
    let relative_path = "../vercel/jade-tracker-vercel/.schemas/MihoResponse.json";
    let relative_path2 = "../vercel/jade-tracker-vercel/.schemas/EstimateCfg.json";
    std::fs::create_dir_all("../vercel/jade-tracker-vercel/.schemas/").unwrap();

    let schema1 = schema_for!(MihoResponse);
    let output = serde_json::to_string_pretty(&schema1).unwrap();

    let schema2 = schema_for!(EstimateCfg);
    let output2 = serde_json::to_string_pretty(&schema2).unwrap();

    std::fs::write(relative_path, output).unwrap();
    std::fs::write(relative_path2, output2).unwrap();
}
