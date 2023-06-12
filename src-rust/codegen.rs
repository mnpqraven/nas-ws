use nas_ws::routes::honkai::mhy_api::types::MihoResponse;
use schemars::schema_for;
use specta::{ts::*, *};

fn main() {
    // ../vercel/jade-tracker-vercel/src/bindings
    let relative_path = "../vercel/jade-tracker-vercel/.schemas/MihoResponse.json";
    let schema = schema_for!(MihoResponse);
    let output = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    std::fs::write(relative_path, output).unwrap();
}
