use std::env;

use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming, transport::Server};

use stalin::{EchoRequest, EchoResponse, SquareRequest, SquareResponse, SumRequest, SumResponse};
use stalin::hello_service_server::{HelloService, HelloServiceServer};

mod stalin;

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl HelloService for MyGreeter {
    async fn square(&self, request: Request<SquareRequest>) -> Result<Response<SquareResponse>, Status> {
        Ok(Response::new(SquareResponse {
            result: request.get_ref().value * request.get_ref().value
        }))
    }

    async fn sum(&self, request: Request<Streaming<SumRequest>>) -> Result<Response<SumResponse>, Status> {
        let mut SUM = 0;
        let mut stream = request.into_inner();
        let mut message = String::from("");
        while let Some(req) = stream.message().await? {
            SUM += req.value
        }
        Ok(Response::new(SumResponse { result: SUM }))
    }

    type EchoWithDelayStream = mpsc::Receiver<Result<EchoResponse, Status>>;

    async fn echo_with_delay(&self, request: Request<EchoRequest>) -> Result<Response<Self::EchoWithDelayStream>, Status> {
        let delay = request.get_ref().clone().delay;
        let message = request.get_ref().clone().message;
        tokio::time::delay_for(std::time::Duration::from_secs(delay as u64));
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            tx.send(Ok(EchoResponse {
                message: format!("Received message: {}", message),
            })).await;
            tokio::time::delay_for(std::time::Duration::from_secs(delay as u64)).await;
            tx.send(Ok(EchoResponse {
                message: format!("Done with message: {} || Slept for {:?}", message, delay),
            })).await;
        });
        Ok(Response::new(rx))
    }

    type ContinuousSumStream = mpsc::Receiver<Result<SumResponse, Status>>;

    async fn continuous_sum(&self, request: Request<Streaming<SumRequest>>) -> Result<Response<Self::ContinuousSumStream>, Status> {
        let mut SUM = 0;
        let mut streamer = request.into_inner();
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(req) = streamer.message().await.unwrap() {
                SUM += req.value;
                tx.send(Ok(SumResponse {
                    result: SUM,
                })).await;
            }
        });
        Ok(Response::new(rx))
    }
}

fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(token) => token.to_str(),
        None => return Err(Status::unauthenticated("Token not found"))
    };
    // do some validation with token here ...
    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let say = MyGreeter::default();
    let ser = HelloServiceServer::with_interceptor(say, interceptor);
    let cert = include_str!("../server.pem");
    let key = include_str!("../server.key");
    let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());

    println!("OUT_DIR is {}", env!("OUT_DIR").to_string());
    println!("Server listening on {}", addr);
    let s = include_str!("../my_ca.pem");
    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls = tonic::transport::ServerTlsConfig::new()
        .identity(id)
        .client_ca_root(ca);
    Server::builder()
        .tls_config(tls)
        .add_service(ser)
        .serve(addr)
        .await?;

    Ok(())
}
