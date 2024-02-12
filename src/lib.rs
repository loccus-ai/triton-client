use crate::grpc_inference_service_client::GrpcInferenceServiceClient;

// code generation

tonic::include_proto!("inference");

// tonic re-export

pub mod tonic {
    pub use tonic::*;
}

// aliases

pub type TritonClient<T> = GrpcInferenceServiceClient<T>;
