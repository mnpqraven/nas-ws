#![feature(trait_alias)]

mod builder;
mod handler;
mod routes;

use axum::Json;
use handler::error::WorkerError;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::routes::{app_router, cron::write_db::write_db};
use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), WorkerError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .init();

    let sched = JobScheduler::new().await?;
    // TODO: derive macro
    sched
        .add(Job::new_repeated_async(
            Duration::from_secs(3600 * 6), // every 6 hours
            |_uuid, _l| {
                Box::pin(async move {
                    let job = write_db().await;
                    match job {
                        Ok(Json(result)) => tracing::info!("{:?}", result),
                        Err(err) => tracing::error!("{}", err.to_string()),
                    }
                })
            },
        )?)
        .await?;
    tracing::info!("cronjob starting...");
    sched.start().await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app_router().into_make_service())
        .await
        .unwrap();

    Ok(())
}
