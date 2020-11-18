use futures::stream::iter;

use stalin::{EchoRequest, EchoResponse, SumRequest, SumResponse, SquareRequest, SquareResponse};
use stalin::hello_service_client::{HelloServiceClient};
use tonic::Request;

mod stalin;

fn get_token() -> String {
    String::from("token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = include_str!("../client.pem");
    let key = include_str!("../client.key");
    let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());
    let s = include_str!("../my_ca.pem");
    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls = tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .tls_config(tls)
        .connect()
        .await?;
    let token = get_token();
    let mut client = HelloServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
        );
        Ok(req)
    });

    /*
     SQUARE
     Request -> Request
     */
    // let request = tonic::Request::new(SquareRequest { value: 5 });
    // let response = client.square(request).await?.into_inner();
    // println!("RESPONSE={:?}", response.result);

    /*
    ECHO DELAY
    Request -> Stream
     */
    // let request = tonic::Request::new(EchoRequest{ message: "QWERTY".to_string(), delay: 5 });
    // let mut response = client.echo_with_delay(request).await?.into_inner();
    // while let Some(res) = response.message().await? {
    //     println!("{}", res.message);
    // }

    /*
    SUM
    Stream -> Request
     */
    // let request = tonic::Request::new(iter(vec![
    //     SumRequest { value: 2 },
    //     SumRequest { value: 2 }
    // ]));
    // let response = client.sum(request).await?.into_inner();
    // println!("SUM = {:?}", response.result);

    /*
    ContinuousSum
    Stream -> Stream
     */
    let request = tonic::Request::new(iter(vec![
        SumRequest { value: 1 },
        SumRequest { value: 2 },
        SumRequest { value: 3 },
        SumRequest { value: 4 },
        SumRequest { value: 5 }
    ]));
    let mut response = client.continuous_sum(request).await?.into_inner();
    while let Some(res) = response.message().await? {
        println!("{:?}", res.result);
    }

    Ok(())
}
