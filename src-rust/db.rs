use libsql_client::Statement;
use nas_ws::builder::get_db_client;
use nas_ws::handler::error::WorkerError;
use nas_ws::runnables::db::{seed_common, seed_table};

// TODO: teardown
async fn teardown() -> Result<(), WorkerError> {
    let client = get_db_client().await?;
    let table_list = vec![
        // "frameworks",
        // "blogs",

        "honkai_avatarSkill",
        "honkai_avatarTrace",
        "honkai_avatarPromotion",
        "honkai_avatarEidolon",
        "honkai_lightConeSkill",
        "honkai_traceMaterial",

        "honkai_lightCone",
        "honkai_eidolon",
        "honkai_item",
        "honkai_skill",

        "honkai_skillType",

        "honkai_itemRarity",
        "honkai_itemSubType",
        "honkai_itemType",

        "honkai_property",

        "honkai_trace",
        "honkai_avatar",

        "honkai_element",
        "honkai_path",
    ];

    client
        .batch(
            table_list
                .into_iter()
                .map(|name| {
                    let st = format!("DROP TABLE IF EXISTS {}", name);
                    Statement::new(st)
                })
                .collect::<Vec<Statement>>(),
        )
        .await?;
    Ok(())
}

// TODO: clap
#[tokio::main]
pub async fn main() -> Result<(), WorkerError> {
    // teardown().await?;
    seed_common().await?;
    seed_table().await?;
    Ok(())
}
