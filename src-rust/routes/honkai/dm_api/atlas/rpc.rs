use self::{
    atlas::{signature_atlas_service_server::SignatureAtlasService, *},
    shared::Empty,
};
use super::{atlas_list, SignatureAtlas};
use crate::routes::endpoint_types::List;
use axum::Json;
use tonic::{Request, Response, Status};

#[allow(non_snake_case)]
pub mod atlas {
    tonic::include_proto!("dm.atlas");
}
pub mod shared {
    tonic::include_proto!("dm.shared");
}

#[tonic::async_trait]
impl SignatureAtlasService for SignatureAtlas {
    async fn list(&self, _request: Request<Empty>) -> Result<Response<SignatureReturns>, Status> {
        let Json(List { list }) = atlas_list().await?;
        let list = SignatureReturns {
            list: list.into_iter().map(|e| e.into()).collect(),
        };

        Ok(Response::new(list))
    }
}

impl From<SignatureAtlas> for SignatureReturn {
    fn from(SignatureAtlas { char_id, lc_id }: SignatureAtlas) -> Self {
        SignatureReturn { char_id, lc_id }
    }
}
