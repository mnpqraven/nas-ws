use self::{
    atlas::{signature_atlas_service_server::*, *},
    shared::*,
};
use super::{atlas_list, SignatureAtlas};
use crate::routes::endpoint_types::List;
use axum::{
    routing::{any_service, MethodRouter},
    Json,
};
use tonic::{Request, Response, Status};
use tonic_web::enable;

#[allow(non_snake_case)]
pub mod atlas {
    tonic::include_proto!("dm.atlas");
}
pub mod shared {
    tonic::include_proto!("dm.shared");
}

#[tonic::async_trait]
impl SignatureAtlasService for SignatureAtlas {
    async fn list(&self, _: Request<()>) -> Result<Response<SignatureReturns>, Status> {
        let Json(List { list }) = atlas_list().await?;
        let list = SignatureReturns {
            list: list.into_iter().map(|e| e.into()).collect(),
        };

        Ok(Response::new(list))
    }

    async fn by_char_id(
        &self,
        request: Request<CharId>,
    ) -> Result<Response<SignatureReturn>, Status> {
        let CharId { char_id } = request.into_inner();
        let Json(List { list }) = atlas_list().await?;
        let ret = list.into_iter().find(|e| e.char_id == char_id);
        match ret {
            Some(ret) => Ok(Response::new(ret.into())),
            None => Err(Status::not_found(char_id.to_string())),
        }
    }
}

impl From<SignatureAtlas> for SignatureReturn {
    fn from(SignatureAtlas { char_id, lc_ids }: SignatureAtlas) -> Self {
        SignatureReturn { char_id, lc_ids }
    }
}

pub fn dm_atlas_route() -> MethodRouter {
    any_service(enable(SignatureAtlasServiceServer::new(
        SignatureAtlas::default(),
    )))
}
