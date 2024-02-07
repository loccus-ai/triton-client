use tonic::transport::Endpoint;
use triton_client::{
    grpc_inference_service_client::GrpcInferenceServiceClient, RepositoryIndexRequest,
};

// constants

const MAX_MESSAGE_SIZE: usize = 32 * 1024 * 1024;

// main

#[tokio::main]
async fn main() {
    let channel = Endpoint::new("http://localhost:8001/")
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut client = GrpcInferenceServiceClient::new(channel)
        .max_encoding_message_size(MAX_MESSAGE_SIZE)
        .max_decoding_message_size(MAX_MESSAGE_SIZE);

    let req = RepositoryIndexRequest {
        repository_name: "".into(),
        ready: false,
    };
    let response = client.repository_index(req).await.unwrap();
    let response = response.into_inner();

    println!("Running models:");
    for model in response.models.iter() {
        println!("{:?}", model);
    }
}
