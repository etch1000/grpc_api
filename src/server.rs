use tonic::{transport::Server, Request, Response, Status};

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct GreeterP {}

#[tonic::async_trait]
impl Greeter for GreeterP {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from : {:#?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse().unwrap();

    let greeter = GreeterP::default();

    println!("GreeterServer Listening on : {address:#?}");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(address)
        .await?;

    Ok(())
}
