use std::env;
use tonic::{transport::Server, Request, Response, Status};
use log::*;

mod pb_hello {
    tonic::include_proto!("hello");
}
use pb_hello::{
    hello_server::{Hello, HelloServer},
    EchoRequest, EchoResponse,
};

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn echo(&self, req: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let params = req.into_inner();

        let res = EchoResponse { ret: params.param };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let port = env::var("PORT").unwrap_or("3000".to_owned());
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();

    let hello = {
        let service = HelloService::default();
        HelloServer::with_interceptor(service, log_intercept)
    };

    info!("start: {}", addr);

    Server::builder()
        .add_service(hello)
        .serve(addr)
        .await?;

    Ok(())
}

fn log_intercept(req: Request<()>) -> Result<Request<()>, Status> {
    info!("{:?}", &req);

    Ok(req)
}
