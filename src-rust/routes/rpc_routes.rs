use self::helloworld::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use axum::{routing::any_service, Router};
use tonic::{Request, Response, Status};
use tonic_web::enable;

use super::honkai::{
    dm_api::atlas::{
        rpc::atlas::signature_atlas_service_server::SignatureAtlasServiceServer, SignatureAtlas,
    },
    jade_estimate::{
        rpc::jadeestimate::jade_estimate_service_server::JadeEstimateServiceServer,
        types::JadeEstimateResponse,
    },
};

pub fn rpc_routes() -> Router {
    let my_greeter = any_service(enable(GreeterServer::new(MyGreeter)));
    let atlas_sv = any_service(enable(SignatureAtlasServiceServer::new(
        SignatureAtlas::default(),
    )));
    let jadeestimate_sv = any_service(enable(JadeEstimateServiceServer::new(
        JadeEstimateResponse::default(),
    )));

    Router::new()
        .route("/helloworld.Greeter/*rpc", my_greeter)
        .route("/dm.atlas.SignatureAtlasService/*rpc", atlas_sv)
        .route("/jadeestimate.JadeEstimateService/*rpc", jadeestimate_sv)
}

#[allow(non_snake_case)]
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
