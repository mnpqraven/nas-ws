mod builder;
mod handler;
mod routes;

use crate::routes::{app_router, cron::dm_repo_clone, cron::write_db};
use handler::error::WorkerError;
use std::{net::SocketAddr, time::Duration};
use tokio_cron_scheduler::{Job, JobScheduler};

#[cfg(debug_assertions)]
const ANSI: bool = true;
#[cfg(not(debug_assertions))]
const ANSI: bool = false;

#[tokio::main]
async fn main() -> Result<(), WorkerError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .with_ansi(ANSI)
        .init();

    let sched = JobScheduler::new().await?;
    sched
        .add(Job::new_repeated_async(
            Duration::from_secs(1800), //half hour
            |_uuid, _l| {
                Box::pin(async move {
                    let _ = dm_repo_clone::execute().await;
                    let _ = dm_repo_clone::chunk_splitter().await;
                })
            },
        )?)
        .await?;

    sched
        .add(Job::new_repeated_async(
            Duration::from_secs(3600 * 6), // every 6 hours
            |_uuid, _l| {
                Box::pin(async move {
                    let _ = write_db::execute().await;
                })
            },
        )?)
        .await?;

    tracing::info!("cronjob starting...");
    sched.start().await?;

    tokio::spawn(async move {
        let _ = dm_repo_clone::execute().await;
        let _ = dm_repo_clone::chunk_splitter().await;
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app_router().into_make_service())
        .await
        .unwrap();

    Ok(())
}
