use futures::stream::iter;

// use stalin::{EchoRequest, EchoResponse, SumRequest, SumResponse, SquareRequest, SquareResponse};
// use stalin::hello_service_client::{HelloServiceClient};
use tonic::Request;
use inference::grpc_inference_service_client::GrpcInferenceServiceClient;
use crate::inference::{ModelInferRequest, ModelInferResponse, InferTensorContents, InferParameter, ModelStatisticsRequest, ModelMetadataRequest, SystemSharedMemoryStatusRequest, ModelReadyRequest};
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
    // let path = "/home/nikita/qqq23Mar3/9ec52f6d-e4f2-4fd3-9d80-3ac07ef663ed-d198_309-0.wav";
    let content_u8 = std::fs::read(Path::new(path)).unwrap();

    let content = vec![160000].iter().map(|x| *x as u8).collect::<Vec<u8>>();
    let content_i32 = vec![160000].iter().map(|x| *x as u8).collect::<Vec<u8>>();

    println!("{:?}", content_u8.len());

    let content_u32 = content_u8.iter().map(|x| *x as u32).collect::<Vec<u32>>();
    let content_i32 = content_u8.iter().map(|x| *x as i32).collect::<Vec<i32>>();

    let channel = tonic::transport::Channel::from_static("http://[::1]:8001")
        .connect()
        .await?;
    let mut client = GrpcInferenceServiceClient::new(channel);

    let mut output_parameters: HashMap<String, InferParameter> = HashMap::new();
    output_parameters.insert(
        "data_type".to_string(),
        InferParameter {
            parameter_choice: Some(ParameterChoice::StringParam("TYPE_FP32".to_string()))
        },
    );
    output_parameters.insert(
        "dims".to_string(),
        InferParameter {
            parameter_choice: Some(ParameterChoice::Int64Param(6))
        },
    );
    output_parameters.insert(
        "label_filename".to_string(),
        InferParameter {
            parameter_choice: Some(ParameterChoice::StringParam("languages.txt".to_string()))
        },
    );
    output_parameters.insert(
        "is_shape_tensor".to_string(),
        InferParameter {
            parameter_choice: Some(ParameterChoice::BoolParam(false))
        },
    );

    let request = tonic::Request::new(iter(vec![
        ModelInferRequest {
            model_name: "encdeclanglabel".to_string(),
            model_version: "1".to_string(),
            id: "12312312312".to_string(),
            parameters: HashMap::new(),
            inputs: vec![
                InferInputTensor {
                    name: "PCM".to_string(),
                    datatype: "TYPE_INT16".to_string(),
                    shape: vec![1, 80000],
                    parameters: Default::default(),
                    contents: None
                    // contents: Some(InferTensorContents {
                    //     bool_contents: vec![],
                    //     int_contents: content,
                    //     int_contents: content_i32.clone(),
                        // int64_contents: vec![],
                        // uint_contents: vec![],
                        // uint_contents: content_u32.clone(),
                        // uint64_contents: vec![],
                        // fp32_contents: vec![],
                        // fp64_contents: vec![],
                        // byte_contents: vec![],
                        // byte_contents: vec![content_u8.clone()],
                    // }),
                },
                InferInputTensor {
                    name: "NUM_OF_SAMPLES".to_string(),
                    datatype: "TYPE_INT32".to_string(),
                    shape: vec![1],
                    parameters: Default::default(),
                    contents: None
                    // contents: Some(InferTensorContents {
                    //     bool_contents: vec![],
                    //     int_contents: vec![content_u32.len().clone() as i32],
                    //     int64_contents: vec![content_u32.len().clone() as i64],
                    //     uint_contents: vec![content_u32.len().clone() as u32],
                    //     uint64_contents: vec![content_u32.len().clone() as u64],
                    //     // int_contents: vec![],
                    //     // int64_contents: vec![],
                    //     // uint_contents: vec![],
                    //     // uint64_contents: vec![],
                    //     fp32_contents: vec![],
                    //     fp64_contents: vec![],
                    //     byte_contents: vec![],
                    // }),
                }
            ],
            outputs: vec![{
                InferRequestedOutputTensor {
                    name: "DECODER_LOGITS".to_string(),
                    parameters: Default::default(),
                }
            }],
            raw_input_contents: vec![
            ],
        },
    ]));
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
    println!();
    println!("###");
    println!();
    let mut response = client.model_stream_infer(request).await?.into_inner();
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

    Ok(())
}
