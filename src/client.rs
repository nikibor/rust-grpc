use futures::stream::iter;

// use stalin::{EchoRequest, EchoResponse, SumRequest, SumResponse, SquareRequest, SquareResponse};
// use stalin::hello_service_client::{HelloServiceClient};
use tonic::Request;
use inference::grpc_inference_service_client::GrpcInferenceServiceClient;
use crate::inference::{ModelInferRequest, ModelInferResponse, InferTensorContents, InferParameter, ModelStatisticsRequest, ModelMetadataRequest, SystemSharedMemoryStatusRequest, ModelReadyRequest, ModelConfigRequest};
use crate::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use std::collections::HashMap;

use crate::inference::infer_parameter::ParameterChoice;
use std::path::Path;

// mod stalin;
mod inference;

fn get_token() -> String {
    String::from("token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "output.wav";
    let content_u8 = std::fs::read(Path::new(path)).unwrap();

    let content = vec![80000].iter().map(|x| *x as u8).collect::<Vec<u8>>();
    let content_i32 = vec![160000].iter().map(|x| *x as u8).collect::<Vec<u8>>();

    println!("{:?}", content_u8.len());

    let content_u32 = content_u8.iter().map(|x| *x as u32).collect::<Vec<u32>>();
    let content_i32 = content_u8.iter().map(|x| *x as i32).collect::<Vec<i32>>();

    let channel = tonic::transport::Channel::from_static("http://[::1]:8001")
        .connect()
        .await?;
    let mut client = GrpcInferenceServiceClient::new(channel);

    let response = client.model_statistics(ModelStatisticsRequest {
        name: "encdeclanglabel".to_string(),
        version: "1".to_string(),
    }).await?.into_inner();
    println!("{:?}", response);


    let response = client.model_metadata(ModelMetadataRequest {
        name: "encdeclanglabel".to_string(),
        version: "1".to_string(),
    }).await?.into_inner();
    println!("{:?}", response);

    let response = client.model_ready(ModelReadyRequest {
        name: "encdeclanglabel".to_string(),
        version: "1".to_string(),
    }).await?.into_inner();
    println!("{:?}", response);


    let model_config = client.model_config(ModelConfigRequest {
        name: "encdeclanglabel".to_string(),
        version: "1".to_string(),
    }).await?.into_inner();
    println!("Model config {:?}", model_config);
    let config = model_config.config.unwrap();


    println!();
    println!("###");
    println!();
    let mut response = client.model_stream_infer(iter(vec![ModelInferRequest {
        model_name: "encdeclanglabel".to_string(),
        model_version: "1".to_string(),
        id: "1".to_string(),
        parameters: HashMap::new(),
        inputs: vec![
            InferInputTensor {
                name: "PCM".to_string(),
                datatype: "INT16".to_string(),
                shape: vec![80000],
                parameters: Default::default(),
                contents: None,
            },
            InferInputTensor {
                name: "NUM_OF_SAMPLES".to_string(),
                datatype: "INT32".to_string(),
                shape: vec![-1, 1],
                parameters: Default::default(),
                contents: None,
            }
        ],
        outputs: vec![{
            InferRequestedOutputTensor {
                name: "DECODER_LOGITS".to_string(),
                parameters: Default::default(),
            }
        }],
        raw_input_contents: vec![
            content.clone(),
            content.len().to_be_bytes().to_vec()
        ],
    }])).await?.into_inner();
    while let Some(res) = response.message().await? {
        match res.infer_response {
            None => {
                println!("{:?}", res.error_message);
            }
            Some(data) => {
                println!("DATA:: {:?}", data);
            }
        }
    }

    let mut response = client.model_infer(ModelInferRequest {
        model_name: "encdeclanglabel".to_string(),
        model_version: "1".to_string(),
        id: "1".to_string(),
        parameters: HashMap::new(),
        inputs: vec![
            InferInputTensor {
                name: "PCM".to_string(),
                datatype: "INT16".to_string(),
                shape: vec![80000],
                parameters: Default::default(),
                contents: None,
            },
            InferInputTensor {
                name: "NUM_OF_SAMPLES".to_string(),
                datatype: "INT32".to_string(),
                shape: vec![-1, 1],
                parameters: Default::default(),
                contents: None,
            }
        ],
        outputs: vec![{
            InferRequestedOutputTensor {
                name: "DECODER_LOGITS".to_string(),
                parameters: Default::default(),
            }
        }],
        raw_input_contents: vec![
            content.clone(),
            content.len().to_be_bytes().to_vec()
        ],
    }).await?.into_inner();
    println!("{:?}", response);

    Ok(())
}
