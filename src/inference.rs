// tonic::include_proto!("inference");

///@@
///@@.. cpp:var:: message ServerLiveRequest
///@@
///@@   Request message for ServerLive.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerLiveRequest {}
///@@
///@@.. cpp:var:: message ServerLiveResponse
///@@
///@@   Response message for ServerLive.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerLiveResponse {
    ///@@
    ///@@  .. cpp:var:: bool live
    ///@@
    ///@@     True if the inference server is live, false it not live.
    ///@@
    #[prost(bool, tag = "1")]
    pub live: bool,
}
///@@
///@@.. cpp:var:: message ServerReadyRequest
///@@
///@@   Request message for ServerReady.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerReadyRequest {}
///@@
///@@.. cpp:var:: message ServerReadyResponse
///@@
///@@   Response message for ServerReady.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerReadyResponse {
    ///@@
    ///@@  .. cpp:var:: bool ready
    ///@@
    ///@@     True if the inference server is ready, false it not ready.
    ///@@
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
///@@
///@@.. cpp:var:: message ModelReadyRequest
///@@
///@@   Request message for ModelReady.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReadyRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the model to check for readiness.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string version
    ///@@
    ///@@     The version of the model to check for readiness. If not given the
    ///@@     server will choose a version based on the model and internal policy.
    ///@@
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
///@@
///@@.. cpp:var:: message ModelReadyResponse
///@@
///@@   Response message for ModelReady.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReadyResponse {
    ///@@
    ///@@  .. cpp:var:: bool ready
    ///@@
    ///@@     True if the model is ready, false it not ready.
    ///@@
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
///@@
///@@.. cpp:var:: message ServerMetadataRequest
///@@
///@@   Request message for ServerMetadata.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadataRequest {}
///@@
///@@.. cpp:var:: message ServerMetadataResponse
///@@
///@@   Response message for ServerMetadata.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadataResponse {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The server name.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@
    ///@@  .. cpp:var:: string version
    ///@@
    ///@@     The server version.
    ///@@
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    ///@@
    ///@@  .. cpp:var:: string extensions (repeated)
    ///@@
    ///@@     The extensions supported by the server.
    ///@@
    #[prost(string, repeated, tag = "3")]
    pub extensions: ::std::vec::Vec<std::string::String>,
}
///@@
///@@.. cpp:var:: message ModelMetadataRequest
///@@
///@@   Request message for ModelMetadata.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelMetadataRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the model.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string version
    ///@@
    ///@@     The version of the model to check for readiness. If not
    ///@@     given the server will choose a version based on the
    ///@@     model and internal policy.
    ///@@
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
///@@
///@@.. cpp:var:: message ModelMetadataResponse
///@@
///@@   Response message for ModelMetadata.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelMetadataResponse {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The model name.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@
    ///@@  .. cpp:var:: string versions (repeated)
    ///@@
    ///@@     The versions of the model.
    ///@@
    #[prost(string, repeated, tag = "2")]
    pub versions: ::std::vec::Vec<std::string::String>,
    ///@@
    ///@@  .. cpp:var:: string platform
    ///@@
    ///@@     The model's platform.
    ///@@
    #[prost(string, tag = "3")]
    pub platform: std::string::String,
    ///@@
    ///@@  .. cpp:var:: TensorMetadata inputs (repeated)
    ///@@
    ///@@     The model's inputs.
    ///@@
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::std::vec::Vec<model_metadata_response::TensorMetadata>,
    ///@@
    ///@@  .. cpp:var:: TensorMetadata outputs (repeated)
    ///@@
    ///@@     The model's outputs.
    ///@@
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::std::vec::Vec<model_metadata_response::TensorMetadata>,
}
pub mod model_metadata_response {
    ///@@
    ///@@  .. cpp:var:: message TensorMetadata
    ///@@
    ///@@     Metadata for a tensor.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TensorMetadata {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The tensor name.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@
        ///@@    .. cpp:var:: string datatype
        ///@@
        ///@@       The tensor data type.
        ///@@
        #[prost(string, tag = "2")]
        pub datatype: std::string::String,
        ///@@
        ///@@    .. cpp:var:: int64 shape (repeated)
        ///@@
        ///@@       The tensor shape. A variable-size dimension is represented
        ///@@       by a -1 value.
        ///@@
        #[prost(int64, repeated, tag = "3")]
        pub shape: ::std::vec::Vec<i64>,
    }
}
///@@
///@@.. cpp:var:: message InferParameter
///@@
///@@   An inference parameter value.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferParameter {
    ///@@  .. cpp:var:: oneof parameter_choice
    ///@@
    ///@@     The parameter value can be a string, an int64 or
    ///@@     a boolean
    ///@@
    #[prost(oneof = "infer_parameter::ParameterChoice", tags = "1, 2, 3")]
    pub parameter_choice: ::std::option::Option<infer_parameter::ParameterChoice>,
}
pub mod infer_parameter {
    ///@@  .. cpp:var:: oneof parameter_choice
    ///@@
    ///@@     The parameter value can be a string, an int64 or
    ///@@     a boolean
    ///@@
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ParameterChoice {
        ///@@    .. cpp:var:: bool bool_param
        ///@@
        ///@@       A boolean parameter value.
        ///@@
        #[prost(bool, tag = "1")]
        BoolParam(bool),
        ///@@    .. cpp:var:: int64 int64_param
        ///@@
        ///@@       An int64 parameter value.
        ///@@
        #[prost(int64, tag = "2")]
        Int64Param(i64),
        ///@@    .. cpp:var:: string string_param
        ///@@
        ///@@       A string parameter value.
        ///@@
        #[prost(string, tag = "3")]
        StringParam(std::string::String),
    }
}
///@@
///@@.. cpp:var:: message InferTensorContents
///@@
///@@   The data contained in a tensor represented by the repeated type
///@@   that matches the tensor's data type. Protobuf oneof is not used
///@@   because oneofs cannot contain repeated fields.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferTensorContents {
    ///@@
    ///@@  .. cpp:var:: bool bool_contents (repeated)
    ///@@
    ///@@     Representation for BOOL data type. The size must match what is
    ///@@     expected by the tensor's shape. The contents must be the flattened,
    ///@@     one-dimensional, row-major order of the tensor elements.
    ///@@
    #[prost(bool, repeated, tag = "1")]
    pub bool_contents: ::std::vec::Vec<bool>,
    ///@@
    ///@@  .. cpp:var:: int32 int_contents (repeated)
    ///@@
    ///@@     Representation for INT8, INT16, and INT32 data types. The size
    ///@@     must match what is expected by the tensor's shape. The contents
    ///@@     must be the flattened, one-dimensional, row-major order of the
    ///@@     tensor elements.
    ///@@
    #[prost(int32, repeated, tag = "2")]
    pub int_contents: ::std::vec::Vec<i32>,
    ///@@
    ///@@  .. cpp:var:: int64 int64_contents (repeated)
    ///@@
    ///@@     Representation for INT64 data types. The size must match what
    ///@@     is expected by the tensor's shape. The contents must be the
    ///@@     flattened, one-dimensional, row-major order of the tensor elements.
    ///@@
    #[prost(int64, repeated, tag = "3")]
    pub int64_contents: ::std::vec::Vec<i64>,
    ///@@
    ///@@  .. cpp:var:: uint32 uint_contents (repeated)
    ///@@
    ///@@     Representation for UINT8, UINT16, and UINT32 data types. The size
    ///@@     must match what is expected by the tensor's shape. The contents
    ///@@     must be the flattened, one-dimensional, row-major order of the
    ///@@     tensor elements.
    ///@@
    #[prost(uint32, repeated, tag = "4")]
    pub uint_contents: ::std::vec::Vec<u32>,
    ///@@
    ///@@  .. cpp:var:: uint64 uint64_contents (repeated)
    ///@@
    ///@@     Representation for UINT64 data types. The size must match what
    ///@@     is expected by the tensor's shape. The contents must be the
    ///@@     flattened, one-dimensional, row-major order of the tensor elements.
    ///@@
    #[prost(uint64, repeated, tag = "5")]
    pub uint64_contents: ::std::vec::Vec<u64>,
    ///@@
    ///@@  .. cpp:var:: float fp32_contents (repeated)
    ///@@
    ///@@     Representation for FP32 data type. The size must match what is
    ///@@     expected by the tensor's shape. The contents must be the flattened,
    ///@@     one-dimensional, row-major order of the tensor elements.
    ///@@
    #[prost(float, repeated, tag = "6")]
    pub fp32_contents: ::std::vec::Vec<f32>,
    ///@@
    ///@@  .. cpp:var:: double fp64_contents (repeated)
    ///@@
    ///@@     Representation for FP64 data type. The size must match what is
    ///@@     expected by the tensor's shape. The contents must be the flattened,
    ///@@     one-dimensional, row-major order of the tensor elements.
    ///@@
    #[prost(double, repeated, tag = "7")]
    pub fp64_contents: ::std::vec::Vec<f64>,
    ///@@
    ///@@  .. cpp:var:: bytes byte_contents (repeated)
    ///@@
    ///@@     Representation for BYTES data type. The size must match what is
    ///@@     expected by the tensor's shape. The contents must be the flattened,
    ///@@     one-dimensional, row-major order of the tensor elements.
    ///@@
    #[prost(bytes, repeated, tag = "8")]
    pub byte_contents: ::std::vec::Vec<std::vec::Vec<u8>>,
}
///@@
///@@.. cpp:var:: message ModelInferRequest
///@@
///@@   Request message for ModelInfer.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelInferRequest {
    ///@@  .. cpp:var:: string model_name
    ///@@
    ///@@     The name of the model to use for inferencing.
    ///@@
    #[prost(string, tag = "1")]
    pub model_name: std::string::String,
    ///@@  .. cpp:var:: string model_version
    ///@@
    ///@@     The version of the model to use for inference. If not
    ///@@     given the latest/most-recent version of the model is used.
    ///@@
    #[prost(string, tag = "2")]
    pub model_version: std::string::String,
    ///@@  .. cpp:var:: string id
    ///@@
    ///@@     Optional identifier for the request. If specified will be
    ///@@     returned in the response.
    ///@@
    #[prost(string, tag = "3")]
    pub id: std::string::String,
    ///@@  .. cpp:var:: map<string,InferParameter> parameters
    ///@@
    ///@@     Optional inference parameters.
    ///@@
    #[prost(map = "string, message", tag = "4")]
    pub parameters: ::std::collections::HashMap<std::string::String, InferParameter>,
    ///@@
    ///@@  .. cpp:var:: InferInputTensor inputs (repeated)
    ///@@
    ///@@     The input tensors for the inference.
    ///@@
    #[prost(message, repeated, tag = "5")]
    pub inputs: ::std::vec::Vec<model_infer_request::InferInputTensor>,
    ///@@
    ///@@  .. cpp:var:: InferRequestedOutputTensor outputs (repeated)
    ///@@
    ///@@     The requested output tensors for the inference. Optional, if not
    ///@@     specified all outputs specified in the model config will be
    ///@@     returned.
    ///@@
    #[prost(message, repeated, tag = "6")]
    pub outputs: ::std::vec::Vec<model_infer_request::InferRequestedOutputTensor>,
    ///@@
    ///@@  .. cpp:var:: bytes raw_input_contents
    ///@@
    ///@@     The data contained in an input tensor can be represented in
    ///@@     "raw" bytes form or in the repeated type that matches the
    ///@@     tensor's data type. Using the "raw" bytes form will
    ///@@     typically allow higher performance due to the way protobuf
    ///@@     allocation and reuse interacts with GRPC. For example, see
    ///@@     https://github.com/grpc/grpc/issues/23231.
    ///@@
    ///@@     To use the raw representation 'raw_input_contents' must be
    ///@@     initialized with data for each tensor in the same order as
    ///@@     'inputs'. For each tensor, the size of this content must
    ///@@     match what is expected by the tensor's shape and data
    ///@@     type. The raw data must be the flattened, one-dimensional,
    ///@@     row-major order of the tensor elements without any stride
    ///@@     or padding between the elements. Note that the FP16 data
    ///@@     type must be represented as raw content as there is no
    ///@@     specific data type for a 16-bit float type.
    ///@@
    ///@@     If this field is specified then InferInputTensor::contents
    ///@@     must not be specified for any input tensor.
    ///@@
    #[prost(bytes, repeated, tag = "7")]
    pub raw_input_contents: ::std::vec::Vec<std::vec::Vec<u8>>,
}
pub mod model_infer_request {
    ///@@
    ///@@  .. cpp:var:: message InferInputTensor
    ///@@
    ///@@     An input tensor for an inference request.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferInputTensor {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The tensor name.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@
        ///@@    .. cpp:var:: string datatype
        ///@@
        ///@@       The tensor data type.
        ///@@
        #[prost(string, tag = "2")]
        pub datatype: std::string::String,
        ///@@
        ///@@    .. cpp:var:: int64 shape (repeated)
        ///@@
        ///@@       The tensor shape.
        ///@@
        #[prost(int64, repeated, tag = "3")]
        pub shape: ::std::vec::Vec<i64>,
        ///@@    .. cpp:var:: map<string,InferParameter> parameters
        ///@@
        ///@@       Optional inference input tensor parameters.
        ///@@
        #[prost(map = "string, message", tag = "4")]
        pub parameters: ::std::collections::HashMap<std::string::String, super::InferParameter>,
        ///@@    .. cpp:var:: InferTensorContents contents
        ///@@
        ///@@       The tensor contents using a data-type format. This field
        ///@@       must not be specified if tensor contents are being specified
        ///@@       in ModelInferRequest.raw_input_contents.
        ///@@
        #[prost(message, optional, tag = "5")]
        pub contents: ::std::option::Option<super::InferTensorContents>,
    }
    ///@@
    ///@@  .. cpp:var:: message InferRequestedOutputTensor
    ///@@
    ///@@     An output tensor requested for an inference request.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferRequestedOutputTensor {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The tensor name.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@    .. cpp:var:: map<string,InferParameter> parameters
        ///@@
        ///@@       Optional requested output tensor parameters.
        ///@@
        #[prost(map = "string, message", tag = "2")]
        pub parameters: ::std::collections::HashMap<std::string::String, super::InferParameter>,
    }
}
///@@
///@@.. cpp:var:: message ModelInferResponse
///@@
///@@   Response message for ModelInfer.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelInferResponse {
    ///@@  .. cpp:var:: string model_name
    ///@@
    ///@@     The name of the model used for inference.
    ///@@
    #[prost(string, tag = "1")]
    pub model_name: std::string::String,
    ///@@  .. cpp:var:: string model_version
    ///@@
    ///@@     The version of the model used for inference.
    ///@@
    #[prost(string, tag = "2")]
    pub model_version: std::string::String,
    ///@@  .. cpp:var:: string id
    ///@@
    ///@@     The id of the inference request if one was specified.
    ///@@
    #[prost(string, tag = "3")]
    pub id: std::string::String,
    ///@@  .. cpp:var:: map<string,InferParameter> parameters
    ///@@
    ///@@     Optional inference response parameters.
    ///@@
    #[prost(map = "string, message", tag = "4")]
    pub parameters: ::std::collections::HashMap<std::string::String, InferParameter>,
    ///@@
    ///@@  .. cpp:var:: InferOutputTensor outputs (repeated)
    ///@@
    ///@@     The output tensors holding inference results.
    ///@@
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::std::vec::Vec<model_infer_response::InferOutputTensor>,
    ///@@
    ///@@  .. cpp:var:: bytes raw_output_contents
    ///@@
    ///@@     The data contained in an output tensor can be represented in
    ///@@     "raw" bytes form or in the repeated type that matches the
    ///@@     tensor's data type. Using the "raw" bytes form will
    ///@@     typically allow higher performance due to the way protobuf
    ///@@     allocation and reuse interacts with GRPC. For example, see
    ///@@     https://github.com/grpc/grpc/issues/23231.
    ///@@
    ///@@     To use the raw representation 'raw_output_contents' must be
    ///@@     initialized with data for each tensor in the same order as
    ///@@     'outputs'. For each tensor, the size of this content must
    ///@@     match what is expected by the tensor's shape and data
    ///@@     type. The raw data must be the flattened, one-dimensional,
    ///@@     row-major order of the tensor elements without any stride
    ///@@     or padding between the elements. Note that the FP16 data
    ///@@     type must be represented as raw content as there is no
    ///@@     specific data type for a 16-bit float type.
    ///@@
    ///@@     If this field is specified then InferOutputTensor::contents
    ///@@     must not be specified for any output tensor.
    ///@@
    #[prost(bytes, repeated, tag = "6")]
    pub raw_output_contents: ::std::vec::Vec<std::vec::Vec<u8>>,
}
pub mod model_infer_response {
    ///@@
    ///@@  .. cpp:var:: message InferOutputTensor
    ///@@
    ///@@     An output tensor returned for an inference request.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferOutputTensor {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The tensor name.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@
        ///@@    .. cpp:var:: string datatype
        ///@@
        ///@@       The tensor data type.
        ///@@
        #[prost(string, tag = "2")]
        pub datatype: std::string::String,
        ///@@
        ///@@    .. cpp:var:: int64 shape (repeated)
        ///@@
        ///@@       The tensor shape.
        ///@@
        #[prost(int64, repeated, tag = "3")]
        pub shape: ::std::vec::Vec<i64>,
        ///@@    .. cpp:var:: map<string,InferParameter> parameters
        ///@@
        ///@@       Optional output tensor parameters.
        ///@@
        #[prost(map = "string, message", tag = "4")]
        pub parameters: ::std::collections::HashMap<std::string::String, super::InferParameter>,
        ///@@    .. cpp:var:: InferTensorContents contents
        ///@@
        ///@@       The tensor contents using a data-type format. This field
        ///@@       must not be specified if tensor contents are being specified
        ///@@       in ModelInferResponse.raw_output_contents.
        ///@@
        #[prost(message, optional, tag = "5")]
        pub contents: ::std::option::Option<super::InferTensorContents>,
    }
}
///@@
///@@.. cpp:var:: message ModelStreamInferResponse
///@@
///@@   Response message for ModelStreamInfer.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelStreamInferResponse {
    ///@@
    ///@@  .. cpp:var:: string error_message
    ///@@
    ///@@     The message describing the error. The empty message
    ///@@     indicates the inference was successful without errors.
    ///@@
    #[prost(string, tag = "1")]
    pub error_message: std::string::String,
    ///@@
    ///@@  .. cpp:var:: ModelInferResponse infer_response
    ///@@
    ///@@     Holds the results of the request.
    ///@@
    #[prost(message, optional, tag = "2")]
    pub infer_response: ::std::option::Option<ModelInferResponse>,
}
///@@
///@@.. cpp:var:: message ModelConfigRequest
///@@
///@@   Request message for ModelConfig.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelConfigRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the model.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string version
    ///@@
    ///@@     The version of the model. If not given the model version
    ///@@     is selected automatically based on the version policy.
    ///@@
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
//@@
//@@.. cpp:var:: message ModelConfigResponse
//@@
//@@   Response message for ModelConfig.
//@@
//message ModelConfigResponse
//{
//  //@@
//  //@@  .. cpp:var:: ModelConfig config
//  //@@
//  //@@     The model configuration.
//  //@@
//  ModelConfig config = 1;
//}

///@@
///@@.. cpp:var:: message ModelStatisticsRequest
///@@
///@@   Request message for ModelStatistics.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelStatisticsRequest {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the model. If not given returns statistics for
    ///@@     all models.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string version
    ///@@
    ///@@     The version of the model. If not given returns statistics for
    ///@@     all model versions.
    ///@@
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
///@@
///@@.. cpp:var:: message StatisticDuration
///@@
///@@   Statistic recording a cumulative duration metric.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticDuration {
    ///@@  .. cpp:var:: uint64 count
    ///@@
    ///@@     Cumulative number of times this metric occurred.
    ///@@
    #[prost(uint64, tag = "1")]
    pub count: u64,
    ///@@  .. cpp:var:: uint64 total_time_ns
    ///@@
    ///@@     Total collected duration of this metric in nanoseconds.
    ///@@
    #[prost(uint64, tag = "2")]
    pub ns: u64,
}
///@@
///@@.. cpp:var:: message InferStatistics
///@@
///@@   Inference statistics.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferStatistics {
    ///@@  .. cpp:var:: StatisticDuration success
    ///@@
    ///@@     Cumulative count and duration for successful inference
    ///@@     request.
    ///@@
    #[prost(message, optional, tag = "1")]
    pub success: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration fail
    ///@@
    ///@@     Cumulative count and duration for failed inference
    ///@@     request.
    ///@@
    #[prost(message, optional, tag = "2")]
    pub fail: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration queue
    ///@@
    ///@@     The count and cumulative duration that inference requests wait in
    ///@@     scheduling or other queues.
    ///@@
    #[prost(message, optional, tag = "3")]
    pub queue: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration compute_input
    ///@@
    ///@@    The count and cumulative duration to prepare input tensor data as
    ///@@    required by the model framework / backend. For example, this duration
    ///@@    should include the time to copy input tensor data to the GPU.
    ///@@
    #[prost(message, optional, tag = "4")]
    pub compute_input: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration compute_infer
    ///@@
    ///@@     The count and cumulative duration to execute the model.
    ///@@
    #[prost(message, optional, tag = "5")]
    pub compute_infer: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration compute_output
    ///@@
    ///@@     The count and cumulative duration to extract output tensor data
    ///@@     produced by the model framework / backend. For example, this duration
    ///@@     should include the time to copy output tensor data from the GPU.
    ///@@
    #[prost(message, optional, tag = "6")]
    pub compute_output: ::std::option::Option<StatisticDuration>,
}
///@@
///@@.. cpp:var:: message InferBatchStatistics
///@@
///@@   Inference batch statistics.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferBatchStatistics {
    ///@@  .. cpp:var:: uint64 batch_size
    ///@@
    ///@@     The size of the batch.
    ///@@
    #[prost(uint64, tag = "1")]
    pub batch_size: u64,
    ///@@  .. cpp:var:: StatisticDuration compute_input
    ///@@
    ///@@     The count and cumulative duration to prepare input tensor data as
    ///@@     required by the model framework / backend with the given batch size.
    ///@@     For example, this duration should include the time to copy input
    ///@@     tensor data to the GPU.
    ///@@
    #[prost(message, optional, tag = "2")]
    pub compute_input: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration compute_infer
    ///@@
    ///@@     The count and cumulative duration to execute the model with the given
    ///@@     batch size.
    ///@@
    #[prost(message, optional, tag = "3")]
    pub compute_infer: ::std::option::Option<StatisticDuration>,
    ///@@  .. cpp:var:: StatisticDuration compute_output
    ///@@
    ///@@     The count and cumulative duration to extract output tensor data
    ///@@     produced by the model framework / backend with the given batch size.
    ///@@     For example, this duration should include the time to copy output
    ///@@     tensor data from the GPU.
    ///@@
    #[prost(message, optional, tag = "4")]
    pub compute_output: ::std::option::Option<StatisticDuration>,
}
///@@
///@@.. cpp:var:: message ModelStatistics
///@@
///@@   Statistics for a specific model and version.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelStatistics {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the model. If not given returns statistics for all
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string version
    ///@@
    ///@@     The version of the model.
    ///@@
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    ///@@  .. cpp:var:: uint64 last_inference
    ///@@
    ///@@     The timestamp of the last inference request made for this model,
    ///@@     as milliseconds since the epoch.
    ///@@
    #[prost(uint64, tag = "3")]
    pub last_inference: u64,
    ///@@  .. cpp:var:: uint64 last_inference
    ///@@
    ///@@     The cumulative count of successful inference requests made for this
    ///@@     model. Each inference in a batched request is counted as an
    ///@@     individual inference. For example, if a client sends a single
    ///@@     inference request with batch size 64, "inference_count" will be
    ///@@     incremented by 64. Similarly, if a clients sends 64 individual
    ///@@     requests each with batch size 1, "inference_count" will be
    ///@@     incremented by 64.
    ///@@
    #[prost(uint64, tag = "4")]
    pub inference_count: u64,
    ///@@  .. cpp:var:: uint64 last_inference
    ///@@
    ///@@     The cumulative count of the number of successful inference executions
    ///@@     performed for the model. When dynamic batching is enabled, a single
    ///@@     model execution can perform inferencing for more than one inference
    ///@@     request. For example, if a clients sends 64 individual requests each
    ///@@     with batch size 1 and the dynamic batcher batches them into a single
    ///@@     large batch for model execution then "execution_count" will be
    ///@@     incremented by 1. If, on the other hand, the dynamic batcher is not
    ///@@     enabled for that each of the 64 individual requests is executed
    ///@@     independently, then "execution_count" will be incremented by 64.
    ///@@
    #[prost(uint64, tag = "5")]
    pub execution_count: u64,
    ///@@  .. cpp:var:: InferStatistics inference_stats
    ///@@
    ///@@     The aggregate statistics for the model/version.
    ///@@
    #[prost(message, optional, tag = "6")]
    pub inference_stats: ::std::option::Option<InferStatistics>,
    ///@@  .. cpp:var:: InferBatchStatistics batch_stats (repeated)
    ///@@
    ///@@     The aggregate statistics for each different batch size that is
    ///@@     executed in the model. The batch statistics indicate how many actual
    ///@@     model executions were performed and show differences due to different
    ///@@     batch size (for example, larger batches typically take longer to
    ///@@     compute).
    ///@@
    #[prost(message, repeated, tag = "7")]
    pub batch_stats: ::std::vec::Vec<InferBatchStatistics>,
}
///@@
///@@.. cpp:var:: message ModelStatisticsResponse
///@@
///@@   Response message for ModelStatistics.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelStatisticsResponse {
    ///@@  .. cpp:var:: ModelStatistics model_stats (repeated)
    ///@@
    ///@@     Statistics for each requested model.
    ///@@
    #[prost(message, repeated, tag = "1")]
    pub model_stats: ::std::vec::Vec<ModelStatistics>,
}
///@@
///@@.. cpp:var:: message RepositoryIndexRequest
///@@
///@@   Request message for RepositoryIndex.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryIndexRequest {
    ///@@  .. cpp:var:: string repository_name
    ///@@
    ///@@     The name of the repository. If empty the index is returned
    ///@@     for all repositories.
    ///@@
    #[prost(string, tag = "1")]
    pub repository_name: std::string::String,
    ///@@  .. cpp:var:: bool ready
    ///@@
    ///@@     If true returned only models currently ready for inferencing.
    ///@@
    #[prost(bool, tag = "2")]
    pub ready: bool,
}
///@@
///@@.. cpp:var:: message RepositoryIndexResponse
///@@
///@@   Response message for RepositoryIndex.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryIndexResponse {
    ///@@
    ///@@  .. cpp:var:: ModelIndex models (repeated)
    ///@@
    ///@@     An index entry for each model.
    ///@@
    #[prost(message, repeated, tag = "1")]
    pub models: ::std::vec::Vec<repository_index_response::ModelIndex>,
}
pub mod repository_index_response {
    ///@@
    ///@@  .. cpp:var:: message ModelIndex
    ///@@
    ///@@     Index entry for a model.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelIndex {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The name of the model.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@    .. cpp:var:: string version
        ///@@
        ///@@       The version of the model.
        ///@@
        #[prost(string, tag = "2")]
        pub version: std::string::String,
        ///@@
        ///@@    .. cpp:var:: string state
        ///@@
        ///@@       The state of the model.
        ///@@
        #[prost(string, tag = "3")]
        pub state: std::string::String,
        ///@@
        ///@@    .. cpp:var:: string reason
        ///@@
        ///@@       The reason, if any, that the model is in the given state.
        ///@@
        #[prost(string, tag = "4")]
        pub reason: std::string::String,
    }
}
///@@
///@@.. cpp:var:: message RepositoryModelLoadRequest
///@@
///@@   Request message for RepositoryModelLoad.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelLoadRequest {
    ///@@  .. cpp:var:: string repository_name
    ///@@
    ///@@     The name of the repository to load from. If empty the model
    ///@@     is loaded from any repository.
    ///@@
    #[prost(string, tag = "1")]
    pub repository_name: std::string::String,
    ///@@  .. cpp:var:: string repository_name
    ///@@
    ///@@     The name of the model to load, or reload.
    ///@@
    #[prost(string, tag = "2")]
    pub model_name: std::string::String,
}
///@@
///@@.. cpp:var:: message RepositoryModelLoadResponse
///@@
///@@   Response message for RepositoryModelLoad.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelLoadResponse {}
///@@
///@@.. cpp:var:: message RepositoryModelUnloadRequest
///@@
///@@   Request message for RepositoryModelUnload.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelUnloadRequest {
    ///@@  .. cpp:var:: string repository_name
    ///@@
    ///@@     The name of the repository from which the model was originally
    ///@@     loaded. If empty the repository is not considered.
    ///@@
    #[prost(string, tag = "1")]
    pub repository_name: std::string::String,
    ///@@  .. cpp:var:: string repository_name
    ///@@
    ///@@     The name of the model to unload.
    ///@@
    #[prost(string, tag = "2")]
    pub model_name: std::string::String,
}
///@@
///@@.. cpp:var:: message RepositoryModelUnloadResponse
///@@
///@@   Response message for RepositoryModelUnload.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelUnloadResponse {}
///@@
///@@.. cpp:var:: message SystemSharedMemoryStatusRequest
///@@
///@@   Request message for SystemSharedMemoryStatus.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSharedMemoryStatusRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the region to get status for. If empty the
    ///@@     status is returned for all registered regions.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
///@@
///@@.. cpp:var:: message SystemSharedMemoryStatusResponse
///@@
///@@   Response message for SystemSharedMemoryStatus.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSharedMemoryStatusResponse {
    ///@@
    ///@@  .. cpp:var:: map<string,RegionStatus> regions
    ///@@
    ///@@     Status for each of the registered regions, indexed by
    ///@@     region name.
    ///@@
    #[prost(map = "string, message", tag = "1")]
    pub regions: ::std::collections::HashMap<
        std::string::String,
        system_shared_memory_status_response::RegionStatus,
    >,
}
pub mod system_shared_memory_status_response {
    ///@@
    ///@@  .. cpp:var:: message RegionStatus
    ///@@
    ///@@     Status for a shared memory region.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegionStatus {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The name for the shared memory region.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@    .. cpp:var:: string shared_memory_key
        ///@@
        ///@@       The key of the underlying memory object that contains the
        ///@@       shared memory region.
        ///@@
        #[prost(string, tag = "2")]
        pub key: std::string::String,
        ///@@    .. cpp:var:: uint64 offset
        ///@@
        ///@@       Offset, in bytes, within the underlying memory object to
        ///@@       the start of the shared memory region.
        ///@@
        #[prost(uint64, tag = "3")]
        pub offset: u64,
        ///@@    .. cpp:var:: uint64 byte_size
        ///@@
        ///@@       Size of the shared memory region, in bytes.
        ///@@
        #[prost(uint64, tag = "4")]
        pub byte_size: u64,
    }
}
///@@
///@@.. cpp:var:: message SystemSharedMemoryRegisterRequest
///@@
///@@   Request message for SystemSharedMemoryRegister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSharedMemoryRegisterRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the region to register.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string shared_memory_key
    ///@@
    ///@@     The key of the underlying memory object that contains the
    ///@@     shared memory region.
    ///@@
    #[prost(string, tag = "2")]
    pub key: std::string::String,
    ///@@  .. cpp:var:: uint64 offset
    ///@@
    ///@@     Offset, in bytes, within the underlying memory object to
    ///@@     the start of the shared memory region.
    ///@@
    #[prost(uint64, tag = "3")]
    pub offset: u64,
    ///@@  .. cpp:var:: uint64 byte_size
    ///@@
    ///@@     Size of the shared memory region, in bytes.
    ///@@
    #[prost(uint64, tag = "4")]
    pub byte_size: u64,
}
///@@
///@@.. cpp:var:: message SystemSharedMemoryRegisterResponse
///@@
///@@   Response message for SystemSharedMemoryRegister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSharedMemoryRegisterResponse {}
///@@
///@@.. cpp:var:: message SystemSharedMemoryUnregisterRequest
///@@
///@@   Request message for SystemSharedMemoryUnregister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSharedMemoryUnregisterRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the system region to unregister. If empty
    ///@@     all system shared-memory regions are unregistered.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
///@@
///@@.. cpp:var:: message SystemSharedMemoryUnregisterResponse
///@@
///@@   Response message for SystemSharedMemoryUnregister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSharedMemoryUnregisterResponse {}
///@@
///@@.. cpp:var:: message CudaSharedMemoryStatusRequest
///@@
///@@   Request message for CudaSharedMemoryStatus.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CudaSharedMemoryStatusRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the region to get status for. If empty the
    ///@@     status is returned for all registered regions.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
///@@
///@@.. cpp:var:: message CudaSharedMemoryStatusResponse
///@@
///@@   Response message for CudaSharedMemoryStatus.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CudaSharedMemoryStatusResponse {
    ///@@
    ///@@  .. cpp:var:: map<string,RegionStatus> regions
    ///@@
    ///@@     Status for each of the registered regions, indexed by
    ///@@     region name.
    ///@@
    #[prost(map = "string, message", tag = "1")]
    pub regions: ::std::collections::HashMap<
        std::string::String,
        cuda_shared_memory_status_response::RegionStatus,
    >,
}
pub mod cuda_shared_memory_status_response {
    ///@@
    ///@@  .. cpp:var:: message RegionStatus
    ///@@
    ///@@     Status for a shared memory region.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegionStatus {
        ///@@
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The name for the shared memory region.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@    .. cpp:var:: uin64 device_id
        ///@@
        ///@@       The GPU device ID where the cudaIPC handle was created.
        ///@@
        #[prost(uint64, tag = "2")]
        pub device_id: u64,
        ///@@    .. cpp:var:: uint64 byte_size
        ///@@
        ///@@       Size of the shared memory region, in bytes.
        ///@@
        #[prost(uint64, tag = "3")]
        pub byte_size: u64,
    }
}
///@@
///@@.. cpp:var:: message CudaSharedMemoryRegisterRequest
///@@
///@@   Request message for CudaSharedMemoryRegister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CudaSharedMemoryRegisterRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the region to register.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: bytes raw_handle
    ///@@
    ///@@     The raw serialized cudaIPC handle.
    ///@@
    #[prost(bytes, tag = "2")]
    pub raw_handle: std::vec::Vec<u8>,
    ///@@  .. cpp:var:: int64 device_id
    ///@@
    ///@@     The GPU device ID on which the cudaIPC handle was created.
    ///@@
    #[prost(int64, tag = "3")]
    pub device_id: i64,
    ///@@  .. cpp:var:: uint64 byte_size
    ///@@
    ///@@     Size of the shared memory block, in bytes.
    ///@@
    #[prost(uint64, tag = "4")]
    pub byte_size: u64,
}
///@@
///@@.. cpp:var:: message CudaSharedMemoryRegisterResponse
///@@
///@@   Response message for CudaSharedMemoryRegister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CudaSharedMemoryRegisterResponse {}
///@@
///@@.. cpp:var:: message CudaSharedMemoryUnregisterRequest
///@@
///@@   Request message for CudaSharedMemoryUnregister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CudaSharedMemoryUnregisterRequest {
    ///@@
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the cuda region to unregister. If empty
    ///@@     all cuda shared-memory regions are unregistered.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
///@@
///@@.. cpp:var:: message CudaSharedMemoryUnregisterResponse
///@@
///@@   Response message for CudaSharedMemoryUnregister.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CudaSharedMemoryUnregisterResponse {}
#[doc = r" Generated client implementations."]
pub mod grpc_inference_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "@@"]
    #[doc = "@@.. cpp:var:: service InferenceService"]
    #[doc = "@@"]
    #[doc = "@@   Inference Server GRPC endpoints."]
    #[doc = "@@"]
    pub struct GrpcInferenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GrpcInferenceServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: std::convert::TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GrpcInferenceServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::ResponseBody: Body + HttpBody + Send + 'static,
            T::Error: Into<StdError>,
            <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = "@@  .. cpp:var:: rpc ServerLive(ServerLiveRequest) returns"]
        #[doc = "@@       (ServerLiveResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Check liveness of the inference server."]
        #[doc = "@@"]
        pub async fn server_live(
            &mut self,
            request: impl tonic::IntoRequest<super::ServerLiveRequest>,
        ) -> Result<tonic::Response<super::ServerLiveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/inference.GRPCInferenceService/ServerLive");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc ServerReady(ServerReadyRequest) returns"]
        #[doc = "@@       (ServerReadyResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Check readiness of the inference server."]
        #[doc = "@@"]
        pub async fn server_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::ServerReadyRequest>,
        ) -> Result<tonic::Response<super::ServerReadyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/inference.GRPCInferenceService/ServerReady");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc ModelReady(ModelReadyRequest) returns"]
        #[doc = "@@       (ModelReadyResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Check readiness of a model in the inference server."]
        #[doc = "@@"]
        pub async fn model_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelReadyRequest>,
        ) -> Result<tonic::Response<super::ModelReadyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/inference.GRPCInferenceService/ModelReady");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc ServerMetadata(ServerMetadataRequest) returns"]
        #[doc = "@@       (ServerMetadataResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get server metadata."]
        #[doc = "@@"]
        pub async fn server_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ServerMetadataRequest>,
        ) -> Result<tonic::Response<super::ServerMetadataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/ServerMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc ModelMetadata(ModelMetadataRequest) returns"]
        #[doc = "@@       (ModelMetadataResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get model metadata."]
        #[doc = "@@"]
        pub async fn model_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelMetadataRequest>,
        ) -> Result<tonic::Response<super::ModelMetadataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/ModelMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc ModelInfer(ModelInferRequest) returns"]
        #[doc = "@@       (ModelInferResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Perform inference using a specific model."]
        #[doc = "@@"]
        pub async fn model_infer(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelInferRequest>,
        ) -> Result<tonic::Response<super::ModelInferResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/inference.GRPCInferenceService/ModelInfer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc ModelStreamInfer(stream ModelInferRequest) returns"]
        #[doc = "@@       (stream ModelStreamInferResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Perform streaming inference."]
        #[doc = "@@"]
        pub async fn model_stream_infer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ModelInferRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ModelStreamInferResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/ModelStreamInfer",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = "@@  .. cpp:var:: rpc ModelStatistics("]
        #[doc = "@@                     ModelStatisticsRequest)"]
        #[doc = "@@                   returns (ModelStatisticsResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get the cumulative inference statistics for a model."]
        #[doc = "@@"]
        pub async fn model_statistics(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelStatisticsRequest>,
        ) -> Result<tonic::Response<super::ModelStatisticsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/ModelStatistics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc RepositoryIndex(RepositoryIndexRequest) returns"]
        #[doc = "@@       (RepositoryIndexResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get the index of model repository contents."]
        #[doc = "@@"]
        pub async fn repository_index(
            &mut self,
            request: impl tonic::IntoRequest<super::RepositoryIndexRequest>,
        ) -> Result<tonic::Response<super::RepositoryIndexResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/RepositoryIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc RepositoryModelLoad(RepositoryModelLoadRequest) returns"]
        #[doc = "@@       (RepositoryModelLoadResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Load or reload a model from a repository."]
        #[doc = "@@"]
        pub async fn repository_model_load(
            &mut self,
            request: impl tonic::IntoRequest<super::RepositoryModelLoadRequest>,
        ) -> Result<tonic::Response<super::RepositoryModelLoadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/RepositoryModelLoad",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc RepositoryModelUnload(RepositoryModelUnloadRequest)"]
        #[doc = "@@       returns (RepositoryModelUnloadResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Unload a model."]
        #[doc = "@@"]
        pub async fn repository_model_unload(
            &mut self,
            request: impl tonic::IntoRequest<super::RepositoryModelUnloadRequest>,
        ) -> Result<tonic::Response<super::RepositoryModelUnloadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/RepositoryModelUnload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc SystemSharedMemoryStatus("]
        #[doc = "@@                     SystemSharedMemoryStatusRequest)"]
        #[doc = "@@                   returns (SystemSharedMemoryStatusRespose)"]
        #[doc = "@@"]
        #[doc = "@@     Get the status of all registered system-shared-memory regions."]
        #[doc = "@@"]
        pub async fn system_shared_memory_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SystemSharedMemoryStatusRequest>,
        ) -> Result<tonic::Response<super::SystemSharedMemoryStatusResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/SystemSharedMemoryStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc SystemSharedMemoryRegister("]
        #[doc = "@@                     SystemSharedMemoryRegisterRequest)"]
        #[doc = "@@                   returns (SystemSharedMemoryRegisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Register a system-shared-memory region."]
        #[doc = "@@"]
        pub async fn system_shared_memory_register(
            &mut self,
            request: impl tonic::IntoRequest<super::SystemSharedMemoryRegisterRequest>,
        ) -> Result<tonic::Response<super::SystemSharedMemoryRegisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/SystemSharedMemoryRegister",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc SystemSharedMemoryUnregister("]
        #[doc = "@@                     SystemSharedMemoryUnregisterRequest)"]
        #[doc = "@@                   returns (SystemSharedMemoryUnregisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Unregister a system-shared-memory region."]
        #[doc = "@@"]
        pub async fn system_shared_memory_unregister(
            &mut self,
            request: impl tonic::IntoRequest<super::SystemSharedMemoryUnregisterRequest>,
        ) -> Result<tonic::Response<super::SystemSharedMemoryUnregisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/SystemSharedMemoryUnregister",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc CudaSharedMemoryStatus("]
        #[doc = "@@                     CudaSharedMemoryStatusRequest)"]
        #[doc = "@@                   returns (CudaSharedMemoryStatusRespose)"]
        #[doc = "@@"]
        #[doc = "@@     Get the status of all registered CUDA-shared-memory regions."]
        #[doc = "@@"]
        pub async fn cuda_shared_memory_status(
            &mut self,
            request: impl tonic::IntoRequest<super::CudaSharedMemoryStatusRequest>,
        ) -> Result<tonic::Response<super::CudaSharedMemoryStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/CudaSharedMemoryStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc CudaSharedMemoryRegister("]
        #[doc = "@@                     CudaSharedMemoryRegisterRequest)"]
        #[doc = "@@                   returns (CudaSharedMemoryRegisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Register a CUDA-shared-memory region."]
        #[doc = "@@"]
        pub async fn cuda_shared_memory_register(
            &mut self,
            request: impl tonic::IntoRequest<super::CudaSharedMemoryRegisterRequest>,
        ) -> Result<tonic::Response<super::CudaSharedMemoryRegisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/CudaSharedMemoryRegister",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "@@  .. cpp:var:: rpc CudaSharedMemoryUnregister("]
        #[doc = "@@                     CudaSharedMemoryUnregisterRequest)"]
        #[doc = "@@                   returns (CudaSharedMemoryUnregisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Unregister a CUDA-shared-memory region."]
        #[doc = "@@"]
        pub async fn cuda_shared_memory_unregister(
            &mut self,
            request: impl tonic::IntoRequest<super::CudaSharedMemoryUnregisterRequest>,
        ) -> Result<tonic::Response<super::CudaSharedMemoryUnregisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inference.GRPCInferenceService/CudaSharedMemoryUnregister",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GrpcInferenceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GrpcInferenceServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GrpcInferenceServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod grpc_inference_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GrpcInferenceServiceServer."]
    #[async_trait]
    pub trait GrpcInferenceService: Send + Sync + 'static {
        #[doc = "@@  .. cpp:var:: rpc ServerLive(ServerLiveRequest) returns"]
        #[doc = "@@       (ServerLiveResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Check liveness of the inference server."]
        #[doc = "@@"]
        async fn server_live(
            &self,
            request: tonic::Request<super::ServerLiveRequest>,
        ) -> Result<tonic::Response<super::ServerLiveResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc ServerReady(ServerReadyRequest) returns"]
        #[doc = "@@       (ServerReadyResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Check readiness of the inference server."]
        #[doc = "@@"]
        async fn server_ready(
            &self,
            request: tonic::Request<super::ServerReadyRequest>,
        ) -> Result<tonic::Response<super::ServerReadyResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc ModelReady(ModelReadyRequest) returns"]
        #[doc = "@@       (ModelReadyResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Check readiness of a model in the inference server."]
        #[doc = "@@"]
        async fn model_ready(
            &self,
            request: tonic::Request<super::ModelReadyRequest>,
        ) -> Result<tonic::Response<super::ModelReadyResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc ServerMetadata(ServerMetadataRequest) returns"]
        #[doc = "@@       (ServerMetadataResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get server metadata."]
        #[doc = "@@"]
        async fn server_metadata(
            &self,
            request: tonic::Request<super::ServerMetadataRequest>,
        ) -> Result<tonic::Response<super::ServerMetadataResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc ModelMetadata(ModelMetadataRequest) returns"]
        #[doc = "@@       (ModelMetadataResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get model metadata."]
        #[doc = "@@"]
        async fn model_metadata(
            &self,
            request: tonic::Request<super::ModelMetadataRequest>,
        ) -> Result<tonic::Response<super::ModelMetadataResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc ModelInfer(ModelInferRequest) returns"]
        #[doc = "@@       (ModelInferResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Perform inference using a specific model."]
        #[doc = "@@"]
        async fn model_infer(
            &self,
            request: tonic::Request<super::ModelInferRequest>,
        ) -> Result<tonic::Response<super::ModelInferResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the ModelStreamInfer method."]
        type ModelStreamInferStream: Stream<Item = Result<super::ModelStreamInferResponse, tonic::Status>>
        + Send
        + Sync
        + 'static;
        #[doc = "@@  .. cpp:var:: rpc ModelStreamInfer(stream ModelInferRequest) returns"]
        #[doc = "@@       (stream ModelStreamInferResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Perform streaming inference."]
        #[doc = "@@"]
        async fn model_stream_infer(
            &self,
            request: tonic::Request<tonic::Streaming<super::ModelInferRequest>>,
        ) -> Result<tonic::Response<Self::ModelStreamInferStream>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc ModelStatistics("]
        #[doc = "@@                     ModelStatisticsRequest)"]
        #[doc = "@@                   returns (ModelStatisticsResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get the cumulative inference statistics for a model."]
        #[doc = "@@"]
        async fn model_statistics(
            &self,
            request: tonic::Request<super::ModelStatisticsRequest>,
        ) -> Result<tonic::Response<super::ModelStatisticsResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc RepositoryIndex(RepositoryIndexRequest) returns"]
        #[doc = "@@       (RepositoryIndexResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get the index of model repository contents."]
        #[doc = "@@"]
        async fn repository_index(
            &self,
            request: tonic::Request<super::RepositoryIndexRequest>,
        ) -> Result<tonic::Response<super::RepositoryIndexResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc RepositoryModelLoad(RepositoryModelLoadRequest) returns"]
        #[doc = "@@       (RepositoryModelLoadResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Load or reload a model from a repository."]
        #[doc = "@@"]
        async fn repository_model_load(
            &self,
            request: tonic::Request<super::RepositoryModelLoadRequest>,
        ) -> Result<tonic::Response<super::RepositoryModelLoadResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc RepositoryModelUnload(RepositoryModelUnloadRequest)"]
        #[doc = "@@       returns (RepositoryModelUnloadResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Unload a model."]
        #[doc = "@@"]
        async fn repository_model_unload(
            &self,
            request: tonic::Request<super::RepositoryModelUnloadRequest>,
        ) -> Result<tonic::Response<super::RepositoryModelUnloadResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc SystemSharedMemoryStatus("]
        #[doc = "@@                     SystemSharedMemoryStatusRequest)"]
        #[doc = "@@                   returns (SystemSharedMemoryStatusRespose)"]
        #[doc = "@@"]
        #[doc = "@@     Get the status of all registered system-shared-memory regions."]
        #[doc = "@@"]
        async fn system_shared_memory_status(
            &self,
            request: tonic::Request<super::SystemSharedMemoryStatusRequest>,
        ) -> Result<tonic::Response<super::SystemSharedMemoryStatusResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc SystemSharedMemoryRegister("]
        #[doc = "@@                     SystemSharedMemoryRegisterRequest)"]
        #[doc = "@@                   returns (SystemSharedMemoryRegisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Register a system-shared-memory region."]
        #[doc = "@@"]
        async fn system_shared_memory_register(
            &self,
            request: tonic::Request<super::SystemSharedMemoryRegisterRequest>,
        ) -> Result<tonic::Response<super::SystemSharedMemoryRegisterResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc SystemSharedMemoryUnregister("]
        #[doc = "@@                     SystemSharedMemoryUnregisterRequest)"]
        #[doc = "@@                   returns (SystemSharedMemoryUnregisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Unregister a system-shared-memory region."]
        #[doc = "@@"]
        async fn system_shared_memory_unregister(
            &self,
            request: tonic::Request<super::SystemSharedMemoryUnregisterRequest>,
        ) -> Result<tonic::Response<super::SystemSharedMemoryUnregisterResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc CudaSharedMemoryStatus("]
        #[doc = "@@                     CudaSharedMemoryStatusRequest)"]
        #[doc = "@@                   returns (CudaSharedMemoryStatusRespose)"]
        #[doc = "@@"]
        #[doc = "@@     Get the status of all registered CUDA-shared-memory regions."]
        #[doc = "@@"]
        async fn cuda_shared_memory_status(
            &self,
            request: tonic::Request<super::CudaSharedMemoryStatusRequest>,
        ) -> Result<tonic::Response<super::CudaSharedMemoryStatusResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc CudaSharedMemoryRegister("]
        #[doc = "@@                     CudaSharedMemoryRegisterRequest)"]
        #[doc = "@@                   returns (CudaSharedMemoryRegisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Register a CUDA-shared-memory region."]
        #[doc = "@@"]
        async fn cuda_shared_memory_register(
            &self,
            request: tonic::Request<super::CudaSharedMemoryRegisterRequest>,
        ) -> Result<tonic::Response<super::CudaSharedMemoryRegisterResponse>, tonic::Status>;
        #[doc = "@@  .. cpp:var:: rpc CudaSharedMemoryUnregister("]
        #[doc = "@@                     CudaSharedMemoryUnregisterRequest)"]
        #[doc = "@@                   returns (CudaSharedMemoryUnregisterResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Unregister a CUDA-shared-memory region."]
        #[doc = "@@"]
        async fn cuda_shared_memory_unregister(
            &self,
            request: tonic::Request<super::CudaSharedMemoryUnregisterRequest>,
        ) -> Result<tonic::Response<super::CudaSharedMemoryUnregisterResponse>, tonic::Status>;
    }
    #[doc = "@@"]
    #[doc = "@@.. cpp:var:: service InferenceService"]
    #[doc = "@@"]
    #[doc = "@@   Inference Server GRPC endpoints."]
    #[doc = "@@"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GrpcInferenceServiceServer<T: GrpcInferenceService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GrpcInferenceService> GrpcInferenceServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GrpcInferenceServiceServer<T>
        where
            T: GrpcInferenceService,
            B: HttpBody + Send + Sync + 'static,
            B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/inference.GRPCInferenceService/ServerLive" => {
                    #[allow(non_camel_case_types)]
                    struct ServerLiveSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ServerLiveRequest> for ServerLiveSvc<T>
                    {
                        type Response = super::ServerLiveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerLiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.server_live(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ServerLiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ServerReady" => {
                    #[allow(non_camel_case_types)]
                    struct ServerReadySvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ServerReadyRequest>
                    for ServerReadySvc<T>
                    {
                        type Response = super::ServerReadyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerReadyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.server_ready(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ServerReadySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ModelReady" => {
                    #[allow(non_camel_case_types)]
                    struct ModelReadySvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ModelReadyRequest> for ModelReadySvc<T>
                    {
                        type Response = super::ModelReadyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelReadyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.model_ready(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModelReadySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ServerMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct ServerMetadataSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ServerMetadataRequest>
                    for ServerMetadataSvc<T>
                    {
                        type Response = super::ServerMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.server_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ServerMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ModelMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct ModelMetadataSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ModelMetadataRequest>
                    for ModelMetadataSvc<T>
                    {
                        type Response = super::ModelMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.model_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModelMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ModelInfer" => {
                    #[allow(non_camel_case_types)]
                    struct ModelInferSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ModelInferRequest> for ModelInferSvc<T>
                    {
                        type Response = super::ModelInferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelInferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.model_infer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModelInferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ModelStreamInfer" => {
                    #[allow(non_camel_case_types)]
                    struct ModelStreamInferSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::StreamingService<super::ModelInferRequest>
                    for ModelStreamInferSvc<T>
                    {
                        type Response = super::ModelStreamInferResponse;
                        type ResponseStream = T::ModelStreamInferStream;
                        type Future =
                        BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ModelInferRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.model_stream_infer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ModelStreamInferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/ModelStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct ModelStatisticsSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ModelStatisticsRequest>
                    for ModelStatisticsSvc<T>
                    {
                        type Response = super::ModelStatisticsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelStatisticsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.model_statistics(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModelStatisticsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/RepositoryIndex" => {
                    #[allow(non_camel_case_types)]
                    struct RepositoryIndexSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::RepositoryIndexRequest>
                    for RepositoryIndexSvc<T>
                    {
                        type Response = super::RepositoryIndexResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepositoryIndexRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.repository_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RepositoryIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/RepositoryModelLoad" => {
                    #[allow(non_camel_case_types)]
                    struct RepositoryModelLoadSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::RepositoryModelLoadRequest>
                    for RepositoryModelLoadSvc<T>
                    {
                        type Response = super::RepositoryModelLoadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepositoryModelLoadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.repository_model_load(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RepositoryModelLoadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/RepositoryModelUnload" => {
                    #[allow(non_camel_case_types)]
                    struct RepositoryModelUnloadSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::RepositoryModelUnloadRequest>
                    for RepositoryModelUnloadSvc<T>
                    {
                        type Response = super::RepositoryModelUnloadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepositoryModelUnloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.repository_model_unload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RepositoryModelUnloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/SystemSharedMemoryStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SystemSharedMemoryStatusSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::SystemSharedMemoryStatusRequest>
                    for SystemSharedMemoryStatusSvc<T>
                    {
                        type Response = super::SystemSharedMemoryStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SystemSharedMemoryStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.system_shared_memory_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SystemSharedMemoryStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/SystemSharedMemoryRegister" => {
                    #[allow(non_camel_case_types)]
                    struct SystemSharedMemoryRegisterSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::SystemSharedMemoryRegisterRequest>
                    for SystemSharedMemoryRegisterSvc<T>
                    {
                        type Response = super::SystemSharedMemoryRegisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SystemSharedMemoryRegisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.system_shared_memory_register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SystemSharedMemoryRegisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/SystemSharedMemoryUnregister" => {
                    #[allow(non_camel_case_types)]
                    struct SystemSharedMemoryUnregisterSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::SystemSharedMemoryUnregisterRequest>
                    for SystemSharedMemoryUnregisterSvc<T>
                    {
                        type Response = super::SystemSharedMemoryUnregisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SystemSharedMemoryUnregisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.system_shared_memory_unregister(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SystemSharedMemoryUnregisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/CudaSharedMemoryStatus" => {
                    #[allow(non_camel_case_types)]
                    struct CudaSharedMemoryStatusSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::CudaSharedMemoryStatusRequest>
                    for CudaSharedMemoryStatusSvc<T>
                    {
                        type Response = super::CudaSharedMemoryStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CudaSharedMemoryStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.cuda_shared_memory_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CudaSharedMemoryStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/CudaSharedMemoryRegister" => {
                    #[allow(non_camel_case_types)]
                    struct CudaSharedMemoryRegisterSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::CudaSharedMemoryRegisterRequest>
                    for CudaSharedMemoryRegisterSvc<T>
                    {
                        type Response = super::CudaSharedMemoryRegisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CudaSharedMemoryRegisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.cuda_shared_memory_register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CudaSharedMemoryRegisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inference.GRPCInferenceService/CudaSharedMemoryUnregister" => {
                    #[allow(non_camel_case_types)]
                    struct CudaSharedMemoryUnregisterSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::CudaSharedMemoryUnregisterRequest>
                    for CudaSharedMemoryUnregisterSvc<T>
                    {
                        type Response = super::CudaSharedMemoryUnregisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CudaSharedMemoryUnregisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.cuda_shared_memory_unregister(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CudaSharedMemoryUnregisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GrpcInferenceService> Clone for GrpcInferenceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GrpcInferenceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GrpcInferenceService> tonic::transport::NamedService for GrpcInferenceServiceServer<T> {
        const NAME: &'static str = "inference.GRPCInferenceService";
    }
}
