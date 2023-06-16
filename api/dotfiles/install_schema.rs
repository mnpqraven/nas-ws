use nas_ws::routes::dotfiles::dotfiles_install_schema_get;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(dotfiles_install_schema_get().await.into())?)
}
