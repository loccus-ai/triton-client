use std::fmt::{self, Display, Formatter};

use crate::grpc_inference_service_client::GrpcInferenceServiceClient;

// code generation

tonic::include_proto!("inference");

// tonic re-export

pub mod tonic {
    pub use tonic::*;
}

// aliases

pub type TritonClient<T> = GrpcInferenceServiceClient<T>;

// datatype

pub enum Datatype {
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Fp16,
    Fp32,
    Fp64,
    String,
    Bf16,
}

impl Display for Datatype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Bool => write!(f, "BOOL"),
            Self::Uint8 => write!(f, "UINT8"),
            Self::Uint16 => write!(f, "UINT16"),
            Self::Uint32 => write!(f, "UINT32"),
            Self::Uint64 => write!(f, "UINT64"),
            Self::Int8 => write!(f, "INT8"),
            Self::Int16 => write!(f, "INT16"),
            Self::Int32 => write!(f, "INT32"),
            Self::Int64 => write!(f, "INT64"),
            Self::Fp16 => write!(f, "FP16"),
            Self::Fp32 => write!(f, "FP32"),
            Self::Fp64 => write!(f, "FP64"),
            Self::String => write!(f, "STRING"),
            Self::Bf16 => write!(f, "BF16"),
        }
    }
}
