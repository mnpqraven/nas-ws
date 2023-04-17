pub mod error;
use axum::Json;
use vercel_runtime::{Body, Response};

/// helper trait to convert an axum response to a vercel runtime response
pub trait FromAxumResponse<TError>
where
    Response<Body>: From<TError>,
    Response<Body>: From<Self>,
    Self: Sized,
{
    fn from_axum(
        result: Result<Json<Self>, TError>,
    ) -> Result<Response<Body>, vercel_runtime::Error> {
        result.map_or_else(|err| Ok(err.into()), |Json(val)| Ok(val.into()))
    }
}
