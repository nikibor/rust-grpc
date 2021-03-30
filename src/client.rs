use futures::stream::iter;

// use stalin::{EchoRequest, EchoResponse, SumRequest, SumResponse, SquareRequest, SquareResponse};
// use stalin::hello_service_client::{HelloServiceClient};
use tonic::Request;
use inference::grpc_inference_service_client::GrpcInferenceServiceClient;
use crate::inference::{ModelInferRequest, ModelInferResponse, InferTensorContents, InferParameter};
use crate::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use std::collections::HashMap;
use crate::inference::infer_parameter::ParameterChoice;

// mod stalin;
mod inference;

fn get_token() -> String {
    String::from("token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cert = include_str!("../client.pem");
    // let key = include_str!("../client.key");
    // let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());
    // let s = include_str!("../my_ca.pem");
    // let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    // let tls = tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);
    let channel = tonic::transport::Channel::from_static("http://[::1]:8001")
        // .tls_config(tls)
        .connect()
        .await?;
    // let token = get_token();
    let mut client = GrpcInferenceServiceClient::new(channel);

    // let mut client = HelloServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //     req.metadata_mut().insert(
    //         "authorization",
    //         tonic::metadata::MetadataValue::from_str(&token).unwrap(),
    //     );
    //     Ok(req)
    // });

    /*
     SQUARE
     Request -> Request
     */
    // let request = tonic::Request::new(inference::ServerLiveRequest{});
    // let response = client.server_live(request).await.into_iter();
    // println!("RESPONSE={:?}", response);

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

    let mut output_parameters: HashMap<String, InferParameter> = HashMap::new();
    output_parameters.insert("data_type".to_string(), InferParameter { parameter_choice: Some(ParameterChoice::Int64Param(6)) });

    let request = tonic::Request::new(iter(vec![
        ModelInferRequest {
            model_name: "encdeclanglabel".to_string(),
            model_version: "-1".to_string(),
            id: "12345".to_string(),
            parameters: Default::default(),
            inputs: vec![
                InferInputTensor {
                    name: "PCM".to_string(),
                    datatype: "TYPE_INT16".to_string(),
                    shape: vec![9],
                    parameters: Default::default(),
                    contents: Some(InferTensorContents {
                        bool_contents: vec![],
                        int_contents: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                        int64_contents: vec![],
                        uint_contents: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                        uint64_contents: vec![],
                        fp32_contents: vec![],
                        fp64_contents: vec![],
                        byte_contents: vec![],
                    }),
                },
                InferInputTensor {
                    name: "NUM_OF_SAMPLES".to_string(),
                    datatype: "TYPE_INT32".to_string(),
                    shape: vec![1],
                    parameters: Default::default(),
                    contents: Some(InferTensorContents {
                        bool_contents: vec![],
                        int_contents: vec![1],
                        int64_contents: vec![],
                        uint_contents: vec![1],
                        uint64_contents: vec![],
                        fp32_contents: vec![],
                        fp64_contents: vec![],
                        byte_contents: vec![],
                    }),
                }
            ],
            outputs: vec![{
                InferRequestedOutputTensor { name: "DECODER_LOGITS".to_string(), parameters: output_parameters.clone() }
            }],
            raw_input_contents: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        },
        ModelInferRequest {
            model_name: "encdeclanglabel".to_string(),
            model_version: "-1".to_string(),
            id: "12323456745".to_string(),
            parameters: Default::default(),
            inputs: vec![
                InferInputTensor {
                    name: "PCM".to_string(),
                    datatype: "TYPE_INT16".to_string(),
                    shape: vec![9],
                    parameters: Default::default(),
                    contents: Some(InferTensorContents {
                        bool_contents: vec![],
                        int_contents: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                        int64_contents: vec![],
                        uint_contents: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                        uint64_contents: vec![],
                        fp32_contents: vec![],
                        fp64_contents: vec![],
                        byte_contents: vec![],
                    }),
                },
                InferInputTensor {
                    name: "NUM_OF_SAMPLES".to_string(),
                    datatype: "TYPE_INT32".to_string(),
                    shape: vec![1],
                    parameters: Default::default(),
                    contents: Some(InferTensorContents {
                        bool_contents: vec![],
                        int_contents: vec![1],
                        int64_contents: vec![],
                        uint_contents: vec![1],
                        uint64_contents: vec![],
                        fp32_contents: vec![],
                        fp64_contents: vec![],
                        byte_contents: vec![],
                    }),
                }
            ],
            outputs: vec![{
                InferRequestedOutputTensor { name: "DECODER_LOGITS".to_string(), parameters: output_parameters.clone() }
            }],
            raw_input_contents: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        },
    ]));
    let mut response = client.model_stream_infer(request).await?.into_inner();
    while let Some(res) = response.message().await? {
        match res.infer_response {
            None => {
                println!("{:?}", res.error_message);
            }
            Some(data) => {
                println!("{:?}", data);
            }
        }
    }

    // let request = tonic::Request::new(iter(vec![
    //     SumRequest { value: 1 },
    //     SumRequest { value: 2 },
    //     SumRequest { value: 3 },
    //     SumRequest { value: 4 },
    //     SumRequest { value: 5 }
    // ]));
    // let mut response = client.continuous_sum(request).await?.into_inner();
    // while let Some(res) = response.message().await? {
    //     println!("{:?}", res.result);
    // }

    Ok(())
}
