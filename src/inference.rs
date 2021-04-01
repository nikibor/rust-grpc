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
///@@
///@@.. cpp:var:: message ModelConfigResponse
///@@
///@@   Response message for ModelConfig.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelConfigResponse {
    ///@@
    ///@@  .. cpp:var:: ModelConfig config
    ///@@
    ///@@     The model configuration.
    ///@@
    #[prost(message, optional, tag = "1")]
    pub config: ::std::option::Option<ModelConfig>,
}
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
///@@
///@@  .. cpp:var:: message ModelRateLimiter
///@@
///@@     The specifications required by the rate limiter to properly
///@@     schedule the inference requests across the different models
///@@     and their instances.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelRateLimiter {
    ///@@  .. cpp:var:: Resource resources (repeated)
    ///@@
    ///@@     The resources required to execute the request on a model instance.
    ///@@     Resources are just names with a corresponding count. The execution
    ///@@     of the instance will be blocked until the specificied resources are
    ///@@     available. By default an instance uses no rate-limiter resources.
    ///@@
    #[prost(message, repeated, tag = "1")]
    pub resources: ::std::vec::Vec<model_rate_limiter::Resource>,
    ///@@  .. cpp:var:: uint32 priority
    ///@@
    ///@@     The weighting value to be used for prioritizing across instances.
    ///@@     An instance with priority 2 will be given 1/2 the number of
    ///@@     scheduling chances as an instance_group with priority 1. The
    ///@@     default priority is 1.
    ///@@
    #[prost(uint32, tag = "2")]
    pub priority: u32,
}
pub mod model_rate_limiter {
    ///@@  .. cpp:var:: message Resource
    ///@@
    ///@@     The resource property.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        ///@@  .. cpp:var:: string name
        ///@@
        ///@@     The name associated with the resource.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@  .. cpp:var:: bool global
        ///@@
        ///@@     Whether or not the resource is global. If true then the resource
        ///@@     is assumed to be shared among the devices otherwise specified
        ///@@     count of the resource is assumed for each device associated
        ///@@     with the instance.
        ///@@
        #[prost(bool, tag = "2")]
        pub global: bool,
        ///@@  .. cpp:var:: uint32 count
        ///@@
        ///@@     The number of resources required for the execution of the model
        ///@@     instance.
        ///@@
        #[prost(uint32, tag = "3")]
        pub count: u32,
    }
}
///@@
///@@.. cpp:var:: message ModelInstanceGroup
///@@
///@@   A group of one or more instances of a model and resources made
///@@   available for those instances.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelInstanceGroup {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     Optional name of this group of instances. If not specified the
    ///@@     name will be formed as <model name>_<group number>. The name of
    ///@@     individual instances will be further formed by a unique instance
    ///@@     number and GPU index:
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: Kind kind
    ///@@
    ///@@     The kind of this instance group. Default is KIND_AUTO. If
    ///@@     KIND_AUTO or KIND_GPU then both 'count' and 'gpu' are valid and
    ///@@     may be specified. If KIND_CPU or KIND_MODEL only 'count' is valid
    ///@@     and 'gpu' cannot be specified.
    ///@@
    #[prost(enumeration = "model_instance_group::Kind", tag = "4")]
    pub kind: i32,
    ///@@  .. cpp:var:: int32 count
    ///@@
    ///@@     For a group assigned to GPU, the number of instances created for
    ///@@     each GPU listed in 'gpus'. For a group assigned to CPU the number
    ///@@     of instances created. Default is 1.
    #[prost(int32, tag = "2")]
    pub count: i32,
    ///@@  .. cpp:var:: ModelRateLimiter rate_limiter
    ///@@
    ///@@     The rate limiter specific settings to be associated with this
    ///@@     instance group. Optional, if not specified no rate limiting
    ///@@     will be applied to this instance group.
    ///@@
    #[prost(message, optional, tag = "6")]
    pub rate_limiter: ::std::option::Option<ModelRateLimiter>,
    ///@@  .. cpp:var:: int32 gpus (repeated)
    ///@@
    ///@@     GPU(s) where instances should be available. For each GPU listed,
    ///@@     'count' instances of the model will be available. Setting 'gpus'
    ///@@     to empty (or not specifying at all) is eqivalent to listing all
    ///@@     available GPUs.
    ///@@
    #[prost(int32, repeated, tag = "3")]
    pub gpus: ::std::vec::Vec<i32>,
    ///@@  .. cpp:var:: string profile (repeated)
    ///@@
    ///@@     For TensorRT models containing multiple optimization profile, this
    ///@@     parameter specifies a set of optimization profiles available to this
    ///@@     instance group. The inference server will choose the optimal profile
    ///@@     based on the shapes of the input tensors. This field should lie
    ///@@     between 0 and <TotalNumberOfOptimizationProfilesInPlanModel> - 1
    ///@@     and be specified only for TensorRT backend, otherwise an error will
    ///@@     be generated. If not specified, the server will select the first
    ///@@     optimization profile by default.
    ///@@
    #[prost(string, repeated, tag = "5")]
    pub profile: ::std::vec::Vec<std::string::String>,
}
pub mod model_instance_group {
    ///@@
    ///@@  .. cpp:enum:: Kind
    ///@@
    ///@@     Kind of this instance group.
    ///@@
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        ///@@    .. cpp:enumerator:: Kind::KIND_AUTO = 0
        ///@@
        ///@@       This instance group represents instances that can run on either
        ///@@       CPU or GPU. If all GPUs listed in 'gpus' are available then
        ///@@       instances will be created on GPU(s), otherwise instances will
        ///@@       be created on CPU.
        ///@@
        Auto = 0,
        ///@@    .. cpp:enumerator:: Kind::KIND_GPU = 1
        ///@@
        ///@@       This instance group represents instances that must run on the
        ///@@       GPU.
        ///@@
        Gpu = 1,
        ///@@    .. cpp:enumerator:: Kind::KIND_CPU = 2
        ///@@
        ///@@       This instance group represents instances that must run on the
        ///@@       CPU.
        ///@@
        Cpu = 2,
        ///@@    .. cpp:enumerator:: Kind::KIND_MODEL = 3
        ///@@
        ///@@       This instance group represents instances that should run on the
        ///@@       CPU and/or GPU(s) as specified by the model or backend itself.
        ///@@       The inference server will not override the model/backend
        ///@@       settings.
        ///@@       Currently, this option is supported only for Tensorflow models.
        ///@@
        Model = 3,
    }
}
///@@
///@@.. cpp:var:: message ModelTensorReshape
///@@
///@@   Reshape specification for input and output tensors.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelTensorReshape {
    ///@@  .. cpp:var:: int64 shape (repeated)
    ///@@
    ///@@     The shape to use for reshaping.
    ///@@
    #[prost(int64, repeated, tag = "1")]
    pub shape: ::std::vec::Vec<i64>,
}
///@@
///@@.. cpp:var:: message ModelInput
///@@
///@@   An input required by the model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelInput {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the input.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: DataType data_type
    ///@@
    ///@@     The data-type of the input.
    ///@@
    #[prost(enumeration = "DataType", tag = "2")]
    pub data_type: i32,
    ///@@  .. cpp:var:: Format format
    ///@@
    ///@@     The format of the input. Optional.
    ///@@
    #[prost(enumeration = "model_input::Format", tag = "3")]
    pub format: i32,
    ///@@  .. cpp:var:: int64 dims (repeated)
    ///@@
    ///@@     The dimensions/shape of the input tensor that must be provided
    ///@@     when invoking the inference API for this model.
    ///@@
    #[prost(int64, repeated, tag = "4")]
    pub dims: ::std::vec::Vec<i64>,
    ///@@  .. cpp:var:: ModelTensorReshape reshape
    ///@@
    ///@@     The shape expected for this input by the backend. The input will
    ///@@     be reshaped to this before being presented to the backend. The
    ///@@     reshape must have the same number of elements as the input shape
    ///@@     specified by 'dims'. Optional.
    ///@@
    #[prost(message, optional, tag = "5")]
    pub reshape: ::std::option::Option<ModelTensorReshape>,
    ///@@  .. cpp:var:: bool is_shape_tensor
    ///@@
    ///@@     Whether or not the input is a shape tensor to the model. This field
    ///@@     is currently supported only for the TensorRT model. An error will be
    ///@@     generated if this specification does not comply with underlying
    ///@@     model.
    ///@@
    #[prost(bool, tag = "6")]
    pub is_shape_tensor: bool,
    ///@@  .. cpp:var:: bool allow_ragged_batch
    ///@@
    ///@@     Whether or not the input is allowed to be "ragged" in a dynamically
    ///@@     created batch. Default is false indicating that two requests will
    ///@@     only be batched if this tensor has the same shape in both requests.
    ///@@     True indicates that two requests can be batched even if this tensor
    ///@@     has a different shape in each request. A true value is currently
    ///@@     supported only for custom models.
    ///@@
    #[prost(bool, tag = "7")]
    pub allow_ragged_batch: bool,
}
pub mod model_input {
    ///@@
    ///@@  .. cpp:enum:: Format
    ///@@
    ///@@     The format for the input.
    ///@@
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        ///@@    .. cpp:enumerator:: Format::FORMAT_NONE = 0
        ///@@
        ///@@       The input has no specific format. This is the default.
        ///@@
        None = 0,
        ///@@    .. cpp:enumerator:: Format::FORMAT_NHWC = 1
        ///@@
        ///@@       HWC image format. Tensors with this format require 3 dimensions
        ///@@       if the model does not support batching (max_batch_size = 0) or 4
        ///@@       dimensions if the model does support batching (max_batch_size
        ///@@       >= 1). In either case the 'dims' below should only specify the
        ///@@       3 non-batch dimensions (i.e. HWC or CHW).
        ///@@
        Nhwc = 1,
        ///@@    .. cpp:enumerator:: Format::FORMAT_NCHW = 2
        ///@@
        ///@@       CHW image format. Tensors with this format require 3 dimensions
        ///@@       if the model does not support batching (max_batch_size = 0) or 4
        ///@@       dimensions if the model does support batching (max_batch_size
        ///@@       >= 1). In either case the 'dims' below should only specify the
        ///@@       3 non-batch dimensions (i.e. HWC or CHW).
        ///@@
        Nchw = 2,
    }
}
///@@
///@@.. cpp:var:: message ModelOutput
///@@
///@@   An output produced by the model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelOutput {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the output.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: DataType data_type
    ///@@
    ///@@     The data-type of the output.
    ///@@
    #[prost(enumeration = "DataType", tag = "2")]
    pub data_type: i32,
    ///@@  .. cpp:var:: int64 dims (repeated)
    ///@@
    ///@@     The dimensions/shape of the output tensor.
    ///@@
    #[prost(int64, repeated, tag = "3")]
    pub dims: ::std::vec::Vec<i64>,
    ///@@  .. cpp:var:: ModelTensorReshape reshape
    ///@@
    ///@@     The shape produced for this output by the backend. The output will
    ///@@     be reshaped from this to the shape specifed in 'dims' before being
    ///@@     returned in the inference response. The reshape must have the same
    ///@@     number of elements as the output shape specified by 'dims'. Optional.
    ///@@
    #[prost(message, optional, tag = "5")]
    pub reshape: ::std::option::Option<ModelTensorReshape>,
    ///@@  .. cpp:var:: string label_filename
    ///@@
    ///@@     The label file associated with this output. Should be specified only
    ///@@     for outputs that represent classifications. Optional.
    ///@@
    #[prost(string, tag = "4")]
    pub label_filename: std::string::String,
    ///@@  .. cpp:var:: bool is_shape_tensor
    ///@@
    ///@@     Whether or not the output is a shape tensor to the model. This field
    ///@@     is currently supported only for the TensorRT model. An error will be
    ///@@     generated if this specification does not comply with underlying
    ///@@     model.
    ///@@
    #[prost(bool, tag = "6")]
    pub is_shape_tensor: bool,
}
///@@  .. cpp:var:: message BatchInput
///@@
///@@     A batch input is an additional input that must be added by
///@@     the backend based on all the requests in a batch.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchInput {
    ///@@    .. cpp:var:: Kind kind
    ///@@
    ///@@       The kind of this batch input.
    ///@@
    #[prost(enumeration = "batch_input::Kind", tag = "1")]
    pub kind: i32,
    ///@@    .. cpp:var:: string target_name (repeated)
    ///@@
    ///@@       The name of the model inputs that the backend will create
    ///@@       for this batch input.
    ///@@
    #[prost(string, repeated, tag = "2")]
    pub target_name: ::std::vec::Vec<std::string::String>,
    ///@@    .. cpp:var:: DataType data_type
    ///@@
    ///@@       The input's datatype. The data type can be TYPE_INT32 or
    ///@@       TYPE_FP32.
    ///@@
    #[prost(enumeration = "DataType", tag = "3")]
    pub data_type: i32,
    ///@@    .. cpp:var:: string source_input (repeated)
    ///@@
    ///@@       The backend derives the value for each batch input from one or
    ///@@       more other inputs. 'source_input' gives the names of those
    ///@@       inputs.
    ///@@
    #[prost(string, repeated, tag = "4")]
    pub source_input: ::std::vec::Vec<std::string::String>,
}
pub mod batch_input {
    ///@@
    ///@@    .. cpp:enum:: Kind
    ///@@
    ///@@       The kind of the batch input.
    ///@@
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        ///@@      .. cpp:enumerator:: Kind::BATCH_ELEMENT_COUNT = 0
        ///@@
        ///@@         The element count of the 'source_input' will be added as
        ///@@         input with shape [1].
        ///@@
        BatchElementCount = 0,
        ///@@      .. cpp:enumerator:: Kind::BATCH_ACCUMULATED_ELEMENT_COUNT = 1
        ///@@
        ///@@         The accumulated element count of the 'source_input' will be
        ///@@         added as input with shape [1]. For example, if there is a
        ///@@         batch of two request, each with 2 elements, an input of value
        ///@@         2 will be added to the first request, and an input of value
        ///@@         4 will be added to the second request.
        ///@@
        BatchAccumulatedElementCount = 1,
        ///@@      .. cpp:enumerator::
        ///@@         Kind::BATCH_ACCUMULATED_ELEMENT_COUNT_WITH_ZERO = 2
        ///@@
        ///@@         The accumulated element count of the 'source_input' will be
        ///@@         added as input with shape [1], except for the first request
        ///@@         in the batch. For the first request in the batch, the input
        ///@@         will have shape [2] where the first element is value 0.
        ///@@
        BatchAccumulatedElementCountWithZero = 2,
        ///@@      .. cpp:enumerator:: Kind::BATCH_MAX_ELEMENT_COUNT_AS_SHAPE = 3
        ///@@
        ///@@         Among the requests in the batch, the max element count of the
        ///@@         'source_input' will be added as input with shape
        ///@@         [max_element_count] for the first request in the batch.
        ///@@         For other requests, such input will be with shape [0].
        ///@@         The data of the tensor will be uninitialized.
        ///@@
        BatchMaxElementCountAsShape = 3,
    }
}
///@@.. cpp:var:: message BatchOutput
///@@
///@@   A batch output is an output produced by the model that must be handled
///@@   differently by the backend based on all the requests in a batch.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOutput {
    ///@@  .. cpp:var:: string target_name (repeated)
    ///@@
    ///@@     The name of the outputs to be produced by this batch output
    ///@@     specification.
    ///@@
    #[prost(string, repeated, tag = "1")]
    pub target_name: ::std::vec::Vec<std::string::String>,
    ///@@  .. cpp:var:: Kind kind
    ///@@
    ///@@     The kind of this batch output.
    ///@@
    #[prost(enumeration = "batch_output::Kind", tag = "2")]
    pub kind: i32,
    ///@@  .. cpp:var:: string source_input (repeated)
    ///@@
    ///@@     The backend derives each batch output from one or more inputs.
    ///@@     'source_input' gives the names of those inputs.
    ///@@
    #[prost(string, repeated, tag = "3")]
    pub source_input: ::std::vec::Vec<std::string::String>,
}
pub mod batch_output {
    ///@@
    ///@@  .. cpp:enum:: Kind
    ///@@
    ///@@     The kind of the batch output.
    ///@@
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        ///@@    .. cpp:enumerator:: Kind::BATCH_SCATTER_WITH_INPUT_SHAPE = 0
        ///@@
        ///@@       The output should be scattered according to the shape of
        ///@@       'source_input'. The dynamic dimension of the output will
        ///@@       be set to the value of the same dimension in the input.
        ///@@
        BatchScatterWithInputShape = 0,
    }
}
///@@
///@@.. cpp:var:: message ModelVersionPolicy
///@@
///@@   Policy indicating which versions of a model should be made
///@@   available by the inference server.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelVersionPolicy {
    ///@@  .. cpp:var:: oneof policy_choice
    ///@@
    ///@@     Each model must implement only a single version policy. The
    ///@@     default policy is 'Latest'.
    ///@@
    #[prost(oneof = "model_version_policy::PolicyChoice", tags = "1, 2, 3")]
    pub policy_choice: ::std::option::Option<model_version_policy::PolicyChoice>,
}
pub mod model_version_policy {
    ///@@  .. cpp:var:: message Latest
    ///@@
    ///@@     Serve only the latest version(s) of a model. This is
    ///@@     the default policy.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Latest {
        ///@@    .. cpp:var:: uint32 num_versions
        ///@@
        ///@@       Serve only the 'num_versions' highest-numbered versions. T
        ///@@       The default value of 'num_versions' is 1, indicating that by
        ///@@       default only the single highest-number version of a
        ///@@       model will be served.
        ///@@
        #[prost(uint32, tag = "1")]
        pub num_versions: u32,
    }
    ///@@  .. cpp:var:: message All
    ///@@
    ///@@     Serve all versions of the model.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {}
    ///@@  .. cpp:var:: message Specific
    ///@@
    ///@@     Serve only specific versions of the model.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Specific {
        ///@@    .. cpp:var:: int64 versions (repeated)
        ///@@
        ///@@       The specific versions of the model that will be served.
        ///@@
        #[prost(int64, repeated, tag = "1")]
        pub versions: ::std::vec::Vec<i64>,
    }
    ///@@  .. cpp:var:: oneof policy_choice
    ///@@
    ///@@     Each model must implement only a single version policy. The
    ///@@     default policy is 'Latest'.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolicyChoice {
        ///@@    .. cpp:var:: Latest latest
        ///@@
        ///@@       Serve only latest version(s) of the model.
        ///@@
        #[prost(message, tag = "1")]
        Latest(Latest),
        ///@@    .. cpp:var:: All all
        ///@@
        ///@@       Serve all versions of the model.
        ///@@
        #[prost(message, tag = "2")]
        All(All),
        ///@@    .. cpp:var:: Specific specific
        ///@@
        ///@@       Serve only specific version(s) of the model.
        ///@@
        #[prost(message, tag = "3")]
        Specific(Specific),
    }
}
///@@
///@@.. cpp:var:: message ModelOptimizationPolicy
///@@
///@@   Optimization settings for a model. These settings control if/how a
///@@   model is optimized and prioritized by the backend framework when
///@@   it is loaded.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelOptimizationPolicy {
    ///@@  .. cpp:var:: Graph graph
    ///@@
    ///@@     The graph optimization setting for the model. Optional.
    ///@@
    #[prost(message, optional, tag = "1")]
    pub graph: ::std::option::Option<model_optimization_policy::Graph>,
    ///@@  .. cpp:var:: ModelPriority priority
    ///@@
    ///@@     The priority setting for the model. Optional.
    ///@@
    #[prost(enumeration = "model_optimization_policy::ModelPriority", tag = "2")]
    pub priority: i32,
    ///@@  .. cpp:var:: Cuda cuda
    ///@@
    ///@@     CUDA-specific optimization settings. Optional.
    ///@@
    #[prost(message, optional, tag = "3")]
    pub cuda: ::std::option::Option<model_optimization_policy::Cuda>,
    ///@@  .. cpp:var:: ExecutionAccelerators execution_accelerators
    ///@@
    ///@@     The accelerators used for the model. Optional.
    ///@@
    #[prost(message, optional, tag = "4")]
    pub execution_accelerators:
    ::std::option::Option<model_optimization_policy::ExecutionAccelerators>,
    ///@@  .. cpp:var:: PinnedMemoryBuffer input_pinned_memory
    ///@@
    ///@@     Use pinned memory buffer when the data transfer for inputs
    ///@@     is between GPU memory and non-pinned system memory.
    ///@@     Default is true.
    ///@@
    #[prost(message, optional, tag = "5")]
    pub input_pinned_memory: ::std::option::Option<model_optimization_policy::PinnedMemoryBuffer>,
    ///@@  .. cpp:var:: PinnedMemoryBuffer output_pinned_memory
    ///@@
    ///@@     Use pinned memory buffer when the data transfer for outputs
    ///@@     is between GPU memory and non-pinned system memory.
    ///@@     Default is true.
    ///@@
    #[prost(message, optional, tag = "6")]
    pub output_pinned_memory: ::std::option::Option<model_optimization_policy::PinnedMemoryBuffer>,
    ///@@  .. cpp:var:: uint32 gather_kernel_buffer_threshold
    ///@@
    ///@@     The backend may use a gather kernel to gather input data if the
    ///@@     device has direct access to the source buffer and the destination
    ///@@     buffer. In such case, the gather kernel will be used only if the
    ///@@     number of buffers to be gathered is greater or equal to
    ///@@     the specifed value. If 0, the gather kernel will be disabled.
    ///@@     Default value is 0.
    ///@@     Currently only recognized by TensorRT backend.
    ///@@
    #[prost(uint32, tag = "7")]
    pub gather_kernel_buffer_threshold: u32,
    ///@@  .. cpp:var:: bool eager_batching
    ///@@
    ///@@     Start preparing the next batch before the model instance is ready
    ///@@     for the next inference. This option can be used to overlap the
    ///@@     batch preparation with model execution, with the trade-off that
    ///@@     the next batch might be smaller than what it could have been.
    ///@@     Default value is false.
    ///@@     Currently only recognized by TensorRT backend.
    ///@@
    #[prost(bool, tag = "8")]
    pub eager_batching: bool,
}
pub mod model_optimization_policy {
    ///@@
    ///@@  .. cpp:var:: message Graph
    ///@@
    ///@@     Enable generic graph optimization of the model. If not specified
    ///@@     the framework's default level of optimization is used. Supports
    ///@@     TensorFlow graphdef and savedmodel and Onnx models. For TensorFlow
    ///@@     causes XLA to be enabled/disabled for the model. For Onnx defaults
    ///@@     to enabling all optimizations, -1 enables only basic optimizations,
    ///@@     +1 enables only basic and extended optimizations.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Graph {
        ///@@    .. cpp:var:: int32 level
        ///@@
        ///@@       The optimization level. Defaults to 0 (zero) if not specified.
        ///@@
        ///@@         - -1: Disabled
        ///@@         -  0: Framework default
        ///@@         -  1+: Enable optimization level (greater values indicate
        ///@@            higher optimization levels)
        ///@@
        #[prost(int32, tag = "1")]
        pub level: i32,
    }
    ///@@
    ///@@  .. cpp:var:: message Cuda
    ///@@
    ///@@     CUDA-specific optimization settings.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cuda {
        ///@@    .. cpp:var:: bool graphs
        ///@@
        ///@@       Use CUDA graphs API to capture model operations and execute
        ///@@       them more efficiently. Default value is false.
        ///@@       Currently only recognized by TensorRT backend.
        ///@@
        #[prost(bool, tag = "1")]
        pub graphs: bool,
        ///@@    .. cpp:var:: bool busy_wait_events
        ///@@
        ///@@       Use busy-waiting to synchronize CUDA events to achieve minimum
        ///@@       latency from event complete to host thread to be notified, with
        ///@@       the cost of high CPU load. Default value is false.
        ///@@       Currently only recognized by TensorRT backend.
        ///@@
        #[prost(bool, tag = "2")]
        pub busy_wait_events: bool,
        ///@@    .. cpp:var:: GraphSpec graph_spec (repeated)
        ///@@
        ///@@       Specification of the CUDA graph to be captured. If not specified
        ///@@       and 'graphs' is true, the default CUDA graphs will be captured
        ///@@       based on model settings.
        ///@@       Currently only recognized by TensorRT backend.
        ///@@
        #[prost(message, repeated, tag = "3")]
        pub graph_spec: ::std::vec::Vec<cuda::GraphSpec>,
        ///@@    .. cpp:var:: bool output_copy_stream
        ///@@
        ///@@       Uses a CUDA stream separate from the inference stream to copy the
        ///@@       output to host. Default value is false.
        ///@@       Currently only recognized by TensorRT backend.
        ///@@
        #[prost(bool, tag = "4")]
        pub output_copy_stream: bool,
    }
    pub mod cuda {
        ///@@    .. cpp:var:: message GraphSpec
        ///@@
        ///@@       Specification of the CUDA graph to be captured.
        ///@@
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GraphSpec {
            ///@@      .. cpp:var:: int32 batch_size
            ///@@
            ///@@         The batch size of the CUDA graph. If 'max_batch_size' is 0,
            ///@@         'batch_size' must be set to 0. Otherwise, 'batch_size' must
            ///@@         be set to value between 1 and 'max_batch_size'.
            ///@@
            #[prost(int32, tag = "1")]
            pub batch_size: i32,
            ///@@      .. cpp:var:: map<string, Shape> input
            ///@@
            ///@@         The specification of the inputs. 'Shape' is the shape of the
            ///@@         input without batching dimension.
            ///@@
            #[prost(map = "string, message", tag = "2")]
            pub input: ::std::collections::HashMap<std::string::String, graph_spec::Shape>,
            ///@@      .. cpp:var:: LowerBound graph_lower_bound
            ///@@
            ///@@         Specify the lower bound of the CUDA graph. Optional.
            ///@@         If specified, the graph can be used for input shapes and
            ///@@         batch sizes that are in closed interval between the lower
            ///@@         bound specification and graph specification. For dynamic
            ///@@         shape model, this allows CUDA graphs to be launched
            ///@@         frequently without capturing all possible shape combinations.
            ///@@         However, using graph for shape combinations different from
            ///@@         the one used for capturing introduces uninitialized data for
            ///@@         execution and it may distort the inference result if
            ///@@         the model is sensitive to uninitialized data.
            ///@@
            #[prost(message, optional, tag = "3")]
            pub graph_lower_bound: ::std::option::Option<graph_spec::LowerBound>,
        }
        pub mod graph_spec {
            ///@@      .. cpp:var:: message Dims
            ///@@
            ///@@         Specification of tensor dimension.
            ///@@
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Shape {
                ///@@        .. cpp:var:: int64 dim (repeated)
                ///@@
                ///@@           The dimension.
                ///@@
                #[prost(int64, repeated, tag = "1")]
                pub dim: ::std::vec::Vec<i64>,
            }
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct LowerBound {
                ///@@      .. cpp:var:: int32 batch_size
                ///@@
                ///@@         The batch size of the CUDA graph. If 'max_batch_size' is 0,
                ///@@         'batch_size' must be set to 0. Otherwise, 'batch_size' must
                ///@@         be set to value between 1 and 'max_batch_size'.
                ///@@
                #[prost(int32, tag = "1")]
                pub batch_size: i32,
                ///@@      .. cpp:var:: map<string, Shape> input
                ///@@
                ///@@         The specification of the inputs. 'Shape' is the shape of
                ///@@         the input without batching dimension.
                ///@@
                #[prost(map = "string, message", tag = "2")]
                pub input: ::std::collections::HashMap<std::string::String, Shape>,
            }
        }
    }
    ///@@
    ///@@  .. cpp:var:: message ExecutionAccelerators
    ///@@
    ///@@     Specify the preferred execution accelerators to be used to execute
    ///@@     the model. Currently only recognized by ONNX Runtime backend and
    ///@@     TensorFlow backend.
    ///@@
    ///@@     For ONNX Runtime backend, it will deploy the model with the execution
    ///@@     accelerators by priority, the priority is determined based on the
    ///@@     order that they are set, i.e. the provider at the front has highest
    ///@@     priority. Overall, the priority will be in the following order:
    ///@@         <gpu_execution_accelerator> (if instance is on GPU)
    ///@@         CUDA Execution Provider     (if instance is on GPU)
    ///@@         <cpu_execution_accelerator>
    ///@@         Default CPU Execution Provider
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionAccelerators {
        ///@@    .. cpp:var:: Accelerator gpu_execution_accelerator (repeated)
        ///@@
        ///@@       The preferred execution provider to be used if the model instance
        ///@@       is deployed on GPU.
        ///@@
        ///@@       For ONNX Runtime backend, possible value is "tensorrt" as name,
        ///@@       and no parameters are required.
        ///@@
        ///@@       For TensorFlow backend, possible values are "tensorrt",
        ///@@       "auto_mixed_precision", "gpu_io".
        ///@@
        ///@@       For "tensorrt", the following parameters can be specified:
        ///@@         "precision_mode": The precision used for optimization.
        ///@@         Allowed values are "FP32" and "FP16". Default value is "FP32".
        ///@@
        ///@@         "max_cached_engines": The maximum number of cached TensorRT
        ///@@         engines in dynamic TensorRT ops. Default value is 100.
        ///@@
        ///@@         "minimum_segment_size": The smallest model subgraph that will
        ///@@         be considered for optimization by TensorRT. Default value is 3.
        ///@@
        ///@@         "max_workspace_size_bytes": The maximum GPU memory the model
        ///@@         can use temporarily during execution. Default value is 1GB.
        ///@@
        ///@@       For "auto_mixed_precision", no parameters are required. If set,
        ///@@       the model will try to use FP16 for better performance.
        ///@@       This optimization can not be set with "tensorrt".
        ///@@
        ///@@       For "gpu_io", no parameters are required. If set, the model will
        ///@@       be executed using TensorFlow Callable API to set input and output
        ///@@       tensors in GPU memory if possible, which can reduce data transfer
        ///@@       overhead if the model is used in ensemble. However, the Callable
        ///@@       object will be created on model creation and it will request all
        ///@@       outputs for every model execution, which may impact the
        ///@@       performance if a request does not require all outputs. This
        ///@@       optimization will only take affect if the model instance is
        ///@@       created with KIND_GPU.
        ///@@
        #[prost(message, repeated, tag = "1")]
        pub gpu_execution_accelerator: ::std::vec::Vec<execution_accelerators::Accelerator>,
        ///@@    .. cpp:var:: Accelerator cpu_execution_accelerator (repeated)
        ///@@
        ///@@       The preferred execution provider to be used if the model instance
        ///@@       is deployed on CPU.
        ///@@
        ///@@       For ONNX Runtime backend, possible value is "openvino" as name,
        ///@@       and no parameters are required.
        ///@@
        #[prost(message, repeated, tag = "2")]
        pub cpu_execution_accelerator: ::std::vec::Vec<execution_accelerators::Accelerator>,
    }
    pub mod execution_accelerators {
        ///@@
        ///@@  .. cpp:var:: message Accelerator
        ///@@
        ///@@     Specify the accelerator to be used to execute the model.
        ///@@     Accelerator with the same name may accept different parameters
        ///@@     depending on the backends.
        ///@@
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Accelerator {
            ///@@    .. cpp:var:: string name
            ///@@
            ///@@       The name of the execution accelerator.
            ///@@
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            ///@@    .. cpp:var:: map<string, string> parameters
            ///@@
            ///@@       Additional paremeters used to configure the accelerator.
            ///@@
            #[prost(map = "string, string", tag = "2")]
            pub parameters: ::std::collections::HashMap<std::string::String, std::string::String>,
        }
    }
    ///@@
    ///@@  .. cpp:var:: message PinnedMemoryBuffer
    ///@@
    ///@@     Specify whether to use a pinned memory buffer when transferring data
    ///@@     between non-pinned system memory and GPU memory. Using a pinned
    ///@@     memory buffer for system from/to GPU transfers will typically provide
    ///@@     increased performance. For example, in the common use case where the
    ///@@     request provides inputs and delivers outputs via non-pinned system
    ///@@     memory, if the model instance accepts GPU IOs, the inputs will be
    ///@@     processed by two copies: from non-pinned system memory to pinned
    ///@@     memory, and from pinned memory to GPU memory. Similarly, pinned
    ///@@     memory will be used for delivering the outputs.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PinnedMemoryBuffer {
        ///@@    .. cpp:var:: bool enable
        ///@@
        ///@@       Use pinned memory buffer. Default is true.
        ///@@
        #[prost(bool, tag = "1")]
        pub enable: bool,
    }
    ///@@
    ///@@  .. cpp:enum:: ModelPriority
    ///@@
    ///@@     Model priorities. A model will be given scheduling and execution
    ///@@     preference over models at lower priorities. Current model
    ///@@     priorities only work for TensorRT models.
    ///@@
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelPriority {
        ///@@    .. cpp:enumerator:: ModelPriority::PRIORITY_DEFAULT = 0
        ///@@
        ///@@       The default model priority.
        ///@@
        PriorityDefault = 0,
        ///@@    .. cpp:enumerator:: ModelPriority::PRIORITY_MAX = 1
        ///@@
        ///@@       The maximum model priority.
        ///@@
        PriorityMax = 1,
        ///@@    .. cpp:enumerator:: ModelPriority::PRIORITY_MIN = 2
        ///@@
        ///@@       The minimum model priority.
        ///@@
        PriorityMin = 2,
    }
}
///@@
///@@.. cpp:var:: message ModelQueuePolicy
///@@
///@@   Queue policy for inference requests.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelQueuePolicy {
    ///@@
    ///@@  .. cpp:var:: TimeoutAction timeout_action
    ///@@
    ///@@     The action applied to timed-out request.
    ///@@     The default action is REJECT.
    ///@@
    #[prost(enumeration = "model_queue_policy::TimeoutAction", tag = "1")]
    pub timeout_action: i32,
    ///@@
    ///@@  .. cpp:var:: uint64 default_timeout_microseconds
    ///@@
    ///@@     The default timeout for every request, in microseconds.
    ///@@     The default value is 0 which indicates that no timeout is set.
    ///@@
    #[prost(uint64, tag = "2")]
    pub default_timeout_microseconds: u64,
    ///@@
    ///@@  .. cpp:var:: bool allow_timeout_override
    ///@@
    ///@@     Whether individual request can override the default timeout value.
    ///@@     When true, individual requests can set a timeout that is less than
    ///@@     the default timeout value but may not increase the timeout.
    ///@@     The default value is false.
    ///@@
    #[prost(bool, tag = "3")]
    pub allow_timeout_override: bool,
    ///@@
    ///@@  .. cpp:var:: uint32 max_queue_size
    ///@@
    ///@@     The maximum queue size for holding requests. A request will be
    ///@@     rejected immediately if it can't be enqueued because the queue is
    ///@@     full. The default value is 0 which indicates that no maximum
    ///@@     queue size is enforced.
    ///@@
    #[prost(uint32, tag = "4")]
    pub max_queue_size: u32,
}
pub mod model_queue_policy {
    ///@@
    ///@@  .. cpp:enum:: TimeoutAction
    ///@@
    ///@@     The action applied to timed-out requests.
    ///@@
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeoutAction {
        ///@@    .. cpp:enumerator:: Action::REJECT = 0
        ///@@
        ///@@       Reject the request and return error message accordingly.
        ///@@
        Reject = 0,
        ///@@    .. cpp:enumerator:: Action::DELAY = 1
        ///@@
        ///@@       Delay the request until all other requests at the same
        ///@@       (or higher) priority levels that have not reached their timeouts
        ///@@       are processed. A delayed request will eventually be processed,
        ///@@       but may be delayed indefinitely due to newly arriving requests.
        ///@@
        Delay = 1,
    }
}
///@@
///@@.. cpp:var:: message ModelDynamicBatching
///@@
///@@   Dynamic batching configuration. These settings control how dynamic
///@@   batching operates for the model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelDynamicBatching {
    ///@@  .. cpp:var:: int32 preferred_batch_size (repeated)
    ///@@
    ///@@     Preferred batch sizes for dynamic batching. If a batch of one of
    ///@@     these sizes can be formed it will be executed immediately.  If
    ///@@     not specified a preferred batch size will be chosen automatically
    ///@@     based on model and GPU characteristics.
    ///@@
    #[prost(int32, repeated, tag = "1")]
    pub preferred_batch_size: ::std::vec::Vec<i32>,
    ///@@  .. cpp:var:: uint64 max_queue_delay_microseconds
    ///@@
    ///@@     The maximum time, in microseconds, a request will be delayed in
    ///@@     the scheduling queue to wait for additional requests for
    ///@@     batching. Default is 0.
    ///@@
    #[prost(uint64, tag = "2")]
    pub max_queue_delay_microseconds: u64,
    ///@@  .. cpp:var:: bool preserve_ordering
    ///@@
    ///@@     Should the dynamic batcher preserve the ordering of responses to
    ///@@     match the order of requests received by the scheduler. Default is
    ///@@     false. If true, the responses will be returned in the same order as
    ///@@     the order of requests sent to the scheduler. If false, the responses
    ///@@     may be returned in arbitrary order. This option is specifically
    ///@@     needed when a sequence of related inference requests (i.e. inference
    ///@@     requests with the same correlation ID) are sent to the dynamic
    ///@@     batcher to ensure that the sequence responses are in the correct
    ///@@     order.
    ///@@
    #[prost(bool, tag = "3")]
    pub preserve_ordering: bool,
    ///@@  .. cpp:var:: uint32 priority_levels
    ///@@
    ///@@     The number of priority levels to be enabled for the model,
    ///@@     the priority level starts from 1 and 1 is the highest priority.
    ///@@     Requests are handled in priority order with all priority 1 requests
    ///@@     processed before priority 2, all priority 2 requests processed before
    ///@@     priority 3, etc. Requests with the same priority level will be
    ///@@     handled in the order that they are received.
    ///@@
    #[prost(uint32, tag = "4")]
    pub priority_levels: u32,
    ///@@  .. cpp:var:: uint32 default_priority_level
    ///@@
    ///@@     The priority level used for requests that don't specify their
    ///@@     priority. The value must be in the range [ 1, 'priority_levels' ].
    ///@@
    #[prost(uint32, tag = "5")]
    pub default_priority_level: u32,
    ///@@  .. cpp:var:: ModelQueuePolicy default_queue_policy
    ///@@
    ///@@     The default queue policy used for requests that don't require
    ///@@     priority handling and requests that specify priority levels where
    ///@@     there is no specific policy given. If not specified, a policy with
    ///@@     default field values will be used.
    ///@@
    #[prost(message, optional, tag = "6")]
    pub default_queue_policy: ::std::option::Option<ModelQueuePolicy>,
    ///@@  .. cpp:var:: map<uint32, ModelQueuePolicy> priority_queue_policy
    ///@@
    ///@@     Specify the queue policy for the priority level. The default queue
    ///@@     policy will be used if a priority level doesn't specify a queue
    ///@@     policy.
    ///@@
    #[prost(map = "uint32, message", tag = "7")]
    pub priority_queue_policy: ::std::collections::HashMap<u32, ModelQueuePolicy>,
}
///@@
///@@.. cpp:var:: message ModelSequenceBatching
///@@
///@@   Sequence batching configuration. These settings control how sequence
///@@   batching operates for the model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelSequenceBatching {
    ///@@  .. cpp:var:: uint64 max_sequence_idle_microseconds
    ///@@
    ///@@     The maximum time, in microseconds, that a sequence is allowed to
    ///@@     be idle before it is aborted. The inference server considers a
    ///@@     sequence idle when it does not have any inference request queued
    ///@@     for the sequence. If this limit is exceeded, the inference server
    ///@@     will free the sequence slot allocated by the sequence and make it
    ///@@     available for another sequence. If not specified (or specified as
    ///@@     zero) a default value of 1000000 (1 second) is used.
    ///@@
    #[prost(uint64, tag = "1")]
    pub max_sequence_idle_microseconds: u64,
    ///@@  .. cpp:var:: ControlInput control_input (repeated)
    ///@@
    ///@@     The model input(s) that the server should use to communicate
    ///@@     sequence start, stop, ready and similar control values to the
    ///@@     model.
    ///@@
    #[prost(message, repeated, tag = "2")]
    pub control_input: ::std::vec::Vec<model_sequence_batching::ControlInput>,
    ///@@  .. cpp:var:: oneof strategy_choice
    ///@@
    ///@@     The strategy used by the sequence batcher. Default strategy
    ///@@     is 'direct'.
    ///@@
    #[prost(oneof = "model_sequence_batching::StrategyChoice", tags = "3, 4")]
    pub strategy_choice: ::std::option::Option<model_sequence_batching::StrategyChoice>,
}
pub mod model_sequence_batching {
    ///@@  .. cpp:var:: message Control
    ///@@
    ///@@     A control is a signal that the sequence batcher uses to
    ///@@     communicate with a backend.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Control {
        ///@@    .. cpp:var:: Kind kind
        ///@@
        ///@@       The kind of this control.
        ///@@
        #[prost(enumeration = "control::Kind", tag = "1")]
        pub kind: i32,
        ///@@    .. cpp:var:: int32 int32_false_true (repeated)
        ///@@
        ///@@       The control's true and false setting is indicated by setting
        ///@@       a value in an int32 tensor. The tensor must be a
        ///@@       1-dimensional tensor with size equal to the batch size of
        ///@@       the request. 'int32_false_true' must have two entries: the
        ///@@       first the false value and the second the true value.
        ///@@
        #[prost(int32, repeated, tag = "2")]
        pub int32_false_true: ::std::vec::Vec<i32>,
        ///@@    .. cpp:var:: float fp32_false_true (repeated)
        ///@@
        ///@@       The control's true and false setting is indicated by setting
        ///@@       a value in a fp32 tensor. The tensor must be a
        ///@@       1-dimensional tensor with size equal to the batch size of
        ///@@       the request. 'fp32_false_true' must have two entries: the
        ///@@       first the false value and the second the true value.
        ///@@
        #[prost(float, repeated, tag = "3")]
        pub fp32_false_true: ::std::vec::Vec<f32>,
        ///@@    .. cpp:var:: DataType data_type
        ///@@
        ///@@       The control's datatype.
        ///@@
        #[prost(enumeration = "super::DataType", tag = "4")]
        pub data_type: i32,
    }
    pub mod control {
        ///@@
        ///@@    .. cpp:enum:: Kind
        ///@@
        ///@@       The kind of the control.
        ///@@
        #[derive(
        Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Kind {
            ///@@      .. cpp:enumerator:: Kind::CONTROL_SEQUENCE_START = 0
            ///@@
            ///@@         A new sequence is/is-not starting. If true a sequence is
            ///@@         starting, if false a sequence is continuing. Must
            ///@@         specify either int32_false_true or fp32_false_true for
            ///@@         this control. This control is optional.
            ///@@
            ControlSequenceStart = 0,
            ///@@      .. cpp:enumerator:: Kind::CONTROL_SEQUENCE_READY = 1
            ///@@
            ///@@         A sequence is/is-not ready for inference. If true the
            ///@@         input tensor data is valid and should be used. If false
            ///@@         the input tensor data is invalid and inferencing should
            ///@@         be "skipped".  Must specify either int32_false_true or
            ///@@         fp32_false_true for this control. This control is optional.
            ///@@
            ControlSequenceReady = 1,
            ///@@      .. cpp:enumerator:: Kind::CONTROL_SEQUENCE_END = 2
            ///@@
            ///@@         A sequence is/is-not ending. If true a sequence is
            ///@@         ending, if false a sequence is continuing. Must
            ///@@         specify either int32_false_true or fp32_false_true for
            ///@@         this control. This control is optional.
            ///@@
            ControlSequenceEnd = 2,
            ///@@      .. cpp:enumerator:: Kind::CONTROL_SEQUENCE_CORRID = 3
            ///@@
            ///@@         The correlation ID of the sequence. The correlation ID
            ///@@         is an uint64_t value that is communicated in whole or
            ///@@         in part by the tensor. The tensor's datatype must be
            ///@@         specified by data_type and must be TYPE_UINT64, TYPE_INT64,
            ///@@         TYPE_UINT32 or TYPE_INT32. If a 32-bit datatype is specified
            ///@@         the correlation ID will be truncated to the low-order 32
            ///@@         bits. This control is optional.
            ///@@
            ControlSequenceCorrid = 3,
        }
    }
    ///@@  .. cpp:var:: message ControlInput
    ///@@
    ///@@     The sequence control values to communicate by a model input.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ControlInput {
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The name of the model input.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@    .. cpp:var:: Control control (repeated)
        ///@@
        ///@@       The control value(s) that should be communicated to the
        ///@@       model using this model input.
        ///@@
        #[prost(message, repeated, tag = "2")]
        pub control: ::std::vec::Vec<Control>,
    }
    ///@@  .. cpp:var:: message StrategyDirect
    ///@@
    ///@@     The sequence batcher uses a specific, unique batch
    ///@@     slot for each sequence. All inference requests in a
    ///@@     sequence are directed to the same batch slot in the same
    ///@@     model instance over the lifetime of the sequence. This
    ///@@     is the default strategy.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StrategyDirect {
        ///@@    .. cpp:var:: uint64 max_queue_delay_microseconds
        ///@@
        ///@@       The maximum time, in microseconds, a candidate request
        ///@@       will be delayed in the sequence batch scheduling queue to
        ///@@       wait for additional requests for batching. Default is 0.
        ///@@
        #[prost(uint64, tag = "1")]
        pub max_queue_delay_microseconds: u64,
        ///@@    .. cpp:var:: float minimum_slot_utilization
        ///@@
        ///@@       The minimum slot utilization that must be satisfied to
        ///@@       execute the batch before 'max_queue_delay_microseconds' expires.
        ///@@       For example, a value of 0.5 indicates that the batch should be
        ///@@       executed as soon as 50% or more of the slots are ready even if
        ///@@       the 'max_queue_delay_microseconds' timeout has not expired.
        ///@@       The default is 0.0, indicating that a batch will be executed
        ///@@       before 'max_queue_delay_microseconds' timeout expires if at least
        ///@@       one batch slot is ready. 'max_queue_delay_microseconds' will be
        ///@@       ignored unless minimum_slot_utilization is set to a non-zero
        ///@@       value.
        ///@@
        #[prost(float, tag = "2")]
        pub minimum_slot_utilization: f32,
    }
    ///@@  .. cpp:var:: message StrategyOldest
    ///@@
    ///@@     The sequence batcher maintains up to 'max_candidate_sequences'
    ///@@     candidate sequences. 'max_candidate_sequences' can be greater
    ///@@     than the model's 'max_batch_size'. For inferencing the batcher
    ///@@     chooses from the candidate sequences up to 'max_batch_size'
    ///@@     inference requests. Requests are chosen in an oldest-first
    ///@@     manner across all candidate sequences. A given sequence is
    ///@@     not guaranteed to be assigned to the same batch slot for
    ///@@     all inference requests of that sequence.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StrategyOldest {
        ///@@    .. cpp:var:: int32 max_candidate_sequences
        ///@@
        ///@@       Maximum number of candidate sequences that the batcher
        ///@@       maintains. Excess seqences are kept in an ordered backlog
        ///@@       and become candidates when existing candidate sequences
        ///@@       complete.
        ///@@
        #[prost(int32, tag = "1")]
        pub max_candidate_sequences: i32,
        ///@@    .. cpp:var:: int32 preferred_batch_size (repeated)
        ///@@
        ///@@       Preferred batch sizes for dynamic batching of candidate
        ///@@       sequences. If a batch of one of these sizes can be formed
        ///@@       it will be executed immediately. If not specified a
        ///@@       preferred batch size will be chosen automatically
        ///@@       based on model and GPU characteristics.
        ///@@
        #[prost(int32, repeated, tag = "2")]
        pub preferred_batch_size: ::std::vec::Vec<i32>,
        ///@@    .. cpp:var:: uint64 max_queue_delay_microseconds
        ///@@
        ///@@       The maximum time, in microseconds, a candidate request
        ///@@       will be delayed in the dynamic batch scheduling queue to
        ///@@       wait for additional requests for batching. Default is 0.
        ///@@
        #[prost(uint64, tag = "3")]
        pub max_queue_delay_microseconds: u64,
    }
    ///@@  .. cpp:var:: oneof strategy_choice
    ///@@
    ///@@     The strategy used by the sequence batcher. Default strategy
    ///@@     is 'direct'.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StrategyChoice {
        ///@@    .. cpp:var:: StrategyDirect direct
        ///@@
        ///@@       StrategyDirect scheduling strategy.
        ///@@
        #[prost(message, tag = "3")]
        Direct(StrategyDirect),
        ///@@    .. cpp:var:: StrategyOldest oldest
        ///@@
        ///@@       StrategyOldest scheduling strategy.
        ///@@
        #[prost(message, tag = "4")]
        Oldest(StrategyOldest),
    }
}
///@@
///@@.. cpp:var:: message ModelEnsembling
///@@
///@@   Model ensembling configuration. These settings specify the models that
///@@   compose the ensemble and how data flows between the models.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelEnsembling {
    ///@@  .. cpp:var:: Step step (repeated)
    ///@@
    ///@@     The models and the input / output mappings used within the ensemble.
    ///@@
    #[prost(message, repeated, tag = "1")]
    pub step: ::std::vec::Vec<model_ensembling::Step>,
}
pub mod model_ensembling {
    ///@@  .. cpp:var:: message Step
    ///@@
    ///@@     Each step specifies a model included in the ensemble,
    ///@@     maps ensemble tensor names to the model input tensors,
    ///@@     and maps model output tensors to ensemble tensor names
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Step {
        ///@@  .. cpp:var:: string model_name
        ///@@
        ///@@     The name of the model to execute for this step of the ensemble.
        ///@@
        #[prost(string, tag = "1")]
        pub model_name: std::string::String,
        ///@@  .. cpp:var:: int64 model_version
        ///@@
        ///@@     The version of the model to use for inference. If -1
        ///@@     the latest/most-recent version of the model is used.
        ///@@
        #[prost(int64, tag = "2")]
        pub model_version: i64,
        ///@@  .. cpp:var:: map<string,string> input_map
        ///@@
        ///@@     Map from name of an input tensor on this step's model to ensemble
        ///@@     tensor name. The ensemble tensor must have the same data type and
        ///@@     shape as the model input. Each model input must be assigned to
        ///@@     one ensemble tensor, but the same ensemble tensor can be assigned
        ///@@     to multiple model inputs.
        ///@@
        #[prost(map = "string, string", tag = "3")]
        pub input_map: ::std::collections::HashMap<std::string::String, std::string::String>,
        ///@@  .. cpp:var:: map<string,string> output_map
        ///@@
        ///@@     Map from name of an output tensor on this step's model to ensemble
        ///@@     tensor name. The data type and shape of the ensemble tensor will
        ///@@     be inferred from the model output. It is optional to assign all
        ///@@     model outputs to ensemble tensors. One ensemble tensor name
        ///@@     can appear in an output map only once.
        ///@@
        #[prost(map = "string, string", tag = "4")]
        pub output_map: ::std::collections::HashMap<std::string::String, std::string::String>,
    }
}
///@@
///@@.. cpp:var:: message ModelParameter
///@@
///@@   A model parameter.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelParameter {
    ///@@  .. cpp:var:: string string_value
    ///@@
    ///@@     The string value of the parameter.
    ///@@
    #[prost(string, tag = "1")]
    pub string_value: std::string::String,
}
///@@
///@@.. cpp:var:: message ModelWarmup
///@@
///@@   Settings used to construct the request sample for model warmup.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelWarmup {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the request sample.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: uint32 batch_size
    ///@@
    ///@@     The batch size of the inference request. This must be >= 1. For
    ///@@     models that don't support batching, batch_size must be 1. If
    ///@@     batch_size > 1, the 'inputs' specified below will be duplicated to
    ///@@     match the batch size requested.
    ///@@
    #[prost(uint32, tag = "2")]
    pub batch_size: u32,
    ///@@  .. cpp:var:: map<string, Input> inputs
    ///@@
    ///@@     The warmup meta data associated with every model input, including
    ///@@     control tensors.
    ///@@
    #[prost(map = "string, message", tag = "3")]
    pub inputs: ::std::collections::HashMap<std::string::String, model_warmup::Input>,
}
pub mod model_warmup {
    ///@@
    ///@@  .. cpp:var:: message Input
    ///@@
    ///@@     Meta data associated with an input.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        ///@@    .. cpp:var:: DataType data_type
        ///@@
        ///@@       The data-type of the input.
        ///@@
        #[prost(enumeration = "super::DataType", tag = "1")]
        pub data_type: i32,
        ///@@    .. cpp:var:: int64 dims (repeated)
        ///@@
        ///@@       The shape of the input tensor, not including the batch dimension.
        ///@@
        #[prost(int64, repeated, tag = "2")]
        pub dims: ::std::vec::Vec<i64>,
        ///@@    .. cpp:var:: oneof input_data_type
        ///@@
        ///@@       Specify how the input data is generated. If the input has STRING
        ///@@       data type and 'random_data' is set, the data generation will fall
        ///@@       back to 'zero_data'.
        ///@@
        #[prost(oneof = "input::InputDataType", tags = "3, 4, 5")]
        pub input_data_type: ::std::option::Option<input::InputDataType>,
    }
    pub mod input {
        ///@@    .. cpp:var:: oneof input_data_type
        ///@@
        ///@@       Specify how the input data is generated. If the input has STRING
        ///@@       data type and 'random_data' is set, the data generation will fall
        ///@@       back to 'zero_data'.
        ///@@
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum InputDataType {
            ///@@
            ///@@    .. cpp:var:: bool zero_data
            ///@@
            ///@@       The identifier for using zeros as input data. Note that the
            ///@@       value of 'zero_data' will not be checked, instead, zero data
            ///@@       will be used as long as the field is set.
            ///@@
            #[prost(bool, tag = "3")]
            ZeroData(bool),
            ///@@
            ///@@    .. cpp:var:: bool random_data
            ///@@
            ///@@       The identifier for using random data as input data. Note that
            ///@@       the value of 'random_data' will not be checked, instead,
            ///@@       random data will be used as long as the field is set.
            ///@@
            #[prost(bool, tag = "4")]
            RandomData(bool),
            ///@@    .. cpp:var:: string input_data_file
            ///@@
            ///@@       The file whose content will be used as raw input data in
            ///@@       row-major order. The file must be provided in a sub-directory
            ///@@       'warmup' under the model directory.
            ///@@
            #[prost(string, tag = "5")]
            InputDataFile(std::string::String),
        }
    }
}
///@@
///@@ .. cpp:var:: message ModelOperations
///@@
///@@    The metadata of libraries providing custom operations for this model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelOperations {
    ///@@  .. cpp:var:: string op_library_filename (repeated)
    ///@@
    ///@@     Optional paths of the libraries providing custom operations for
    ///@@     this model. Valid only for ONNX models.
    ///@@
    #[prost(string, repeated, tag = "1")]
    pub op_library_filename: ::std::vec::Vec<std::string::String>,
}
///@@
///@@ .. cpp:var:: message ModelTransactionPolicy
///@@
///@@    The specification that describes the nature of transactions
///@@    to be expected from the model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelTransactionPolicy {
    ///@@  .. cpp:var:: bool decoupled
    ///@@
    ///@@     Indicates whether responses generated by the model are decoupled with
    ///@@     the requests issued to it, which means the number of responses
    ///@@     generated by model may differ from number of requests issued, and
    ///@@     that the responses may be out of order relative to the order of
    ///@@     requests. The default is false, which means the model will generate
    ///@@     exactly one response for each request.
    ///@@
    #[prost(bool, tag = "1")]
    pub decoupled: bool,
}
///@@
///@@.. cpp:var:: message ModelRepositoryAgents
///@@
///@@   The repository agents for the model.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelRepositoryAgents {
    ///@@
    ///@@  .. cpp:var:: Agent agents (repeated)
    ///@@
    ///@@     The ordered list of agents for the model. These agents will be
    ///@@     invoked in order to respond to repository actions occuring for the
    ///@@     model.
    ///@@
    #[prost(message, repeated, tag = "1")]
    pub agents: ::std::vec::Vec<model_repository_agents::Agent>,
}
pub mod model_repository_agents {
    ///@@
    ///@@  .. cpp:var:: message Agent
    ///@@
    ///@@     A repository agent that should be invoked for the specified
    ///@@     repository actions for this model.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Agent {
        ///@@    .. cpp:var:: string name
        ///@@
        ///@@       The name of the agent.
        ///@@
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        ///@@    .. cpp:var:: map<string, string> parameters
        ///@@
        ///@@       The parameters for the agent.
        ///@@
        #[prost(map = "string, string", tag = "2")]
        pub parameters: ::std::collections::HashMap<std::string::String, std::string::String>,
    }
}
///@@
///@@.. cpp:var:: message ModelConfig
///@@
///@@   A model configuration.
///@@
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelConfig {
    ///@@  .. cpp:var:: string name
    ///@@
    ///@@     The name of the model.
    ///@@
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///@@  .. cpp:var:: string platform
    ///@@
    ///@@     The framework for the model. Possible values are
    ///@@     "tensorrt_plan", "tensorflow_graphdef",
    ///@@     "tensorflow_savedmodel", "onnxruntime_onnx",
    ///@@     "pytorch_libtorch" and "custom".
    ///@@
    #[prost(string, tag = "2")]
    pub platform: std::string::String,
    ///@@  .. cpp:var:: string backend
    ///@@
    ///@@     The backend used by the model.
    ///@@
    #[prost(string, tag = "17")]
    pub backend: std::string::String,
    ///@@  .. cpp:var:: ModelVersionPolicy version_policy
    ///@@
    ///@@     Policy indicating which version(s) of the model will be served.
    ///@@
    #[prost(message, optional, tag = "3")]
    pub version_policy: ::std::option::Option<ModelVersionPolicy>,
    ///@@  .. cpp:var:: int32 max_batch_size
    ///@@
    ///@@     Maximum batch size allowed for inference. This can only decrease
    ///@@     what is allowed by the model itself. A max_batch_size value of 0
    ///@@     indicates that batching is not allowed for the model and the
    ///@@     dimension/shape of the input and output tensors must exactly
    ///@@     match what is specified in the input and output configuration. A
    ///@@     max_batch_size value > 0 indicates that batching is allowed and
    ///@@     so the model expects the input tensors to have an additional
    ///@@     initial dimension for the batching that is not specified in the
    ///@@     input (for example, if the model supports batched inputs of
    ///@@     2-dimensional tensors then the model configuration will specify
    ///@@     the input shape as [ X, Y ] but the model will expect the actual
    ///@@     input tensors to have shape [ N, X, Y ]). For max_batch_size > 0
    ///@@     returned outputs will also have an additional initial dimension
    ///@@     for the batch.
    ///@@
    #[prost(int32, tag = "4")]
    pub max_batch_size: i32,
    ///@@  .. cpp:var:: ModelInput input (repeated)
    ///@@
    ///@@     The inputs request by the model.
    ///@@
    #[prost(message, repeated, tag = "5")]
    pub input: ::std::vec::Vec<ModelInput>,
    ///@@  .. cpp:var:: ModelOutput output (repeated)
    ///@@
    ///@@     The outputs produced by the model.
    ///@@
    #[prost(message, repeated, tag = "6")]
    pub output: ::std::vec::Vec<ModelOutput>,
    ///@@  .. cpp:var:: BatchInput batch_input (repeated)
    ///@@
    ///@@     The model input(s) that the server should use to communicate
    ///@@     batch related values to the model.
    ///@@
    #[prost(message, repeated, tag = "20")]
    pub batch_input: ::std::vec::Vec<BatchInput>,
    ///@@  .. cpp:var:: BatchOutput batch_output (repeated)
    ///@@
    ///@@     The outputs produced by the model that requires special handling
    ///@@     by the model backend.
    ///@@
    #[prost(message, repeated, tag = "21")]
    pub batch_output: ::std::vec::Vec<BatchOutput>,
    ///@@  .. cpp:var:: ModelOptimizationPolicy optimization
    ///@@
    ///@@     Optimization configuration for the model. If not specified
    ///@@     then default optimization policy is used.
    ///@@
    #[prost(message, optional, tag = "12")]
    pub optimization: ::std::option::Option<ModelOptimizationPolicy>,
    ///@@  .. cpp:var:: ModelInstanceGroup instance_group (repeated)
    ///@@
    ///@@     Instances of this model. If not specified, one instance
    ///@@     of the model will be instantiated on each available GPU.
    ///@@
    #[prost(message, repeated, tag = "7")]
    pub instance_group: ::std::vec::Vec<ModelInstanceGroup>,
    ///@@  .. cpp:var:: string default_model_filename
    ///@@
    ///@@     Optional filename of the model file to use if a
    ///@@     compute-capability specific model is not specified in
    ///@@     :cpp:var:`cc_model_filenames`. If not specified the default name
    ///@@     is 'model.graphdef', 'model.savedmodel', 'model.plan' or
    ///@@     'model.pt' depending on the model type.
    ///@@
    #[prost(string, tag = "8")]
    pub default_model_filename: std::string::String,
    ///@@  .. cpp:var:: map<string,string> cc_model_filenames
    ///@@
    ///@@     Optional map from CUDA compute capability to the filename of
    ///@@     the model that supports that compute capability. The filename
    ///@@     refers to a file within the model version directory.
    ///@@
    #[prost(map = "string, string", tag = "9")]
    pub cc_model_filenames: ::std::collections::HashMap<std::string::String, std::string::String>,
    ///@@  .. cpp:var:: map<string,string> metric_tags
    ///@@
    ///@@     Optional metric tags. User-specific key-value pairs for metrics
    ///@@     reported for this model. These tags are applied to the metrics
    ///@@     reported on the HTTP metrics port.
    ///@@
    #[prost(map = "string, string", tag = "10")]
    pub metric_tags: ::std::collections::HashMap<std::string::String, std::string::String>,
    ///@@  .. cpp:var:: map<string,ModelParameter> parameters
    ///@@
    ///@@     Optional model parameters. User-specified parameter values that
    ///@@     are made available to custom backends.
    ///@@
    #[prost(map = "string, message", tag = "14")]
    pub parameters: ::std::collections::HashMap<std::string::String, ModelParameter>,
    ///@@  .. cpp:var:: ModelWarmup model_warmup (repeated)
    ///@@
    ///@@     Warmup setting of this model. If specified, all instances
    ///@@     will be run with the request samples in sequence before
    ///@@     serving the model.
    ///@@     This field can only be specified if the model is not an ensemble
    ///@@     model.
    ///@@
    #[prost(message, repeated, tag = "16")]
    pub model_warmup: ::std::vec::Vec<ModelWarmup>,
    ///@@  .. cpp:var:: ModelOperations model_operations
    ///@@
    ///@@     Optional metadata of the libraries providing custom operations for
    ///@@     this model.
    ///@@
    #[prost(message, optional, tag = "18")]
    pub model_operations: ::std::option::Option<ModelOperations>,
    ///@@  .. cpp:var:: ModelTransactionPolicy model_transaction_policy
    ///@@
    ///@@     Optional specification that describes the nature of transactions
    ///@@     to be expected from the model.
    ///@@
    #[prost(message, optional, tag = "19")]
    pub model_transaction_policy: ::std::option::Option<ModelTransactionPolicy>,
    ///@@  .. cpp:var:: ModelRepositoryAgents model_repository_agents
    ///@@
    ///@@     Optional specification of the agent(s) that should be invoked
    ///@@     with repository actions are performed for this model.
    ///@@
    #[prost(message, optional, tag = "23")]
    pub model_repository_agents: ::std::option::Option<ModelRepositoryAgents>,
    ///@@  .. cpp:var:: oneof scheduling_choice
    ///@@
    ///@@     The scheduling policy for the model. If not specified the
    ///@@     default scheduling policy is used for the model. The default
    ///@@     policy is to execute each inference request independently.
    ///@@
    #[prost(oneof = "model_config::SchedulingChoice", tags = "11, 13, 15")]
    pub scheduling_choice: ::std::option::Option<model_config::SchedulingChoice>,
}
pub mod model_config {
    ///@@  .. cpp:var:: oneof scheduling_choice
    ///@@
    ///@@     The scheduling policy for the model. If not specified the
    ///@@     default scheduling policy is used for the model. The default
    ///@@     policy is to execute each inference request independently.
    ///@@
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SchedulingChoice {
        ///@@    .. cpp:var:: ModelDynamicBatching dynamic_batching
        ///@@
        ///@@       If specified, enables the dynamic-batching scheduling
        ///@@       policy. With dynamic-batching the scheduler may group
        ///@@       together independent requests into a single batch to
        ///@@       improve inference throughput.
        ///@@
        #[prost(message, tag = "11")]
        DynamicBatching(super::ModelDynamicBatching),
        ///@@    .. cpp:var:: ModelSequenceBatching sequence_batching
        ///@@
        ///@@       If specified, enables the sequence-batching scheduling
        ///@@       policy. With sequence-batching, inference requests
        ///@@       with the same correlation ID are routed to the same
        ///@@       model instance. Multiple sequences of inference requests
        ///@@       may be batched together into a single batch to
        ///@@       improve inference throughput.
        ///@@
        #[prost(message, tag = "13")]
        SequenceBatching(super::ModelSequenceBatching),
        ///@@    .. cpp:var:: ModelEnsembling ensemble_scheduling
        ///@@
        ///@@       If specified, enables the model-ensembling scheduling
        ///@@       policy. With model-ensembling, inference requests
        ///@@       will be processed according to the specification, such as an
        ///@@       execution sequence of models. The input specified in this model
        ///@@       config will be the input for the ensemble, and the output
        ///@@       specified will be the output of the ensemble.
        ///@@
        #[prost(message, tag = "15")]
        EnsembleScheduling(super::ModelEnsembling),
    }
}
// Copyright (c) 2018-2020, NVIDIA CORPORATION. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//  * Neither the name of NVIDIA CORPORATION nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
// OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Copyright (c) 2018, TensorFlow Authors. All rights reserved.

//@@.. cpp:namespace:: inference

///@@
///@@.. cpp:enum:: DataType
///@@
///@@   Data types supported for input and output tensors.
///@@
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    ///@@  .. cpp:enumerator:: DataType::INVALID = 0
    TypeInvalid = 0,
    ///@@  .. cpp:enumerator:: DataType::BOOL = 1
    TypeBool = 1,
    ///@@  .. cpp:enumerator:: DataType::UINT8 = 2
    TypeUint8 = 2,
    ///@@  .. cpp:enumerator:: DataType::UINT16 = 3
    TypeUint16 = 3,
    ///@@  .. cpp:enumerator:: DataType::UINT32 = 4
    TypeUint32 = 4,
    ///@@  .. cpp:enumerator:: DataType::UINT64 = 5
    TypeUint64 = 5,
    ///@@  .. cpp:enumerator:: DataType::INT8 = 6
    TypeInt8 = 6,
    ///@@  .. cpp:enumerator:: DataType::INT16 = 7
    TypeInt16 = 7,
    ///@@  .. cpp:enumerator:: DataType::INT32 = 8
    TypeInt32 = 8,
    ///@@  .. cpp:enumerator:: DataType::INT64 = 9
    TypeInt64 = 9,
    ///@@  .. cpp:enumerator:: DataType::FP16 = 10
    TypeFp16 = 10,
    ///@@  .. cpp:enumerator:: DataType::FP32 = 11
    TypeFp32 = 11,
    ///@@  .. cpp:enumerator:: DataType::FP64 = 12
    TypeFp64 = 12,
    ///@@  .. cpp:enumerator:: DataType::STRING = 13
    TypeString = 13,
}
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
        #[doc = "@@  .. cpp:var:: rpc ModelConfig(ModelConfigRequest) returns"]
        #[doc = "@@       (ModelConfigResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get model configuration."]
        #[doc = "@@"]
        pub async fn model_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelConfigRequest>,
        ) -> Result<tonic::Response<super::ModelConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/inference.GRPCInferenceService/ModelConfig");
            self.inner.unary(request.into_request(), path, codec).await
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
        #[doc = "@@  .. cpp:var:: rpc ModelConfig(ModelConfigRequest) returns"]
        #[doc = "@@       (ModelConfigResponse)"]
        #[doc = "@@"]
        #[doc = "@@     Get model configuration."]
        #[doc = "@@"]
        async fn model_config(
            &self,
            request: tonic::Request<super::ModelConfigRequest>,
        ) -> Result<tonic::Response<super::ModelConfigResponse>, tonic::Status>;
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
                "/inference.GRPCInferenceService/ModelConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ModelConfigSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<T: GrpcInferenceService>
                    tonic::server::UnaryService<super::ModelConfigRequest>
                    for ModelConfigSvc<T>
                    {
                        type Response = super::ModelConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.model_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModelConfigSvc(inner);
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
