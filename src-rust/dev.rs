use nas_ws::routes::honkai::utils::patch_date::list_future_patch_date;
use std::error::Error;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let t = list_future_patch_date().await.unwrap();
    debug!("{:?}", t);
    Ok(())
}
