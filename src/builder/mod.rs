use serde::Deserialize;

mod types;

#[derive(Deserialize)]
pub struct MyParams {
    pub id: String,
    pub name: String,
}
