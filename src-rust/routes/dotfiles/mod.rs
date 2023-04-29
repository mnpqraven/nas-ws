use axum::{routing::get, Router};
use dotfiles_schema::generate_schema;

pub fn dotfiles_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(initial_install_script).post(initial_install_script),
        )
        .route("/install_schema", get(dotfiles_install_schema_get))
}

async fn initial_install_script() -> String {
    "under development".into()
}

pub async fn dotfiles_install_schema_get() -> String {
    generate_schema()
}
