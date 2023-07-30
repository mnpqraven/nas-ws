use self::hello_world::{greeter_server::Greeter, HelloReply, HelloRequest};
use crate::routes::rpc::hello_world::greeter_server::GreeterServer;
use axum::{routing::any_service, Router};
use tonic::{Request, Response, Status};
use tonic_web::enable;

#[allow(non_snake_case)]
pub mod hello_world {
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

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

pub fn rpc_routes() -> Router {
    let my_greeter = enable(GreeterServer::new(MyGreeter));

    Router::new().route("/helloworld.Greeter/*rpc", any_service(my_greeter))
}
