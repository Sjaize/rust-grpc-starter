// Bidirectional streaming gRPC client

use tonic::transport::Channel;

use prg_04_serverstreaming::serverstreaming_pb2_grpc::server_streaming_client::ServerStreamingClient;
use prg_04_serverstreaming::serverstreaming_pb2::Number;

async fn recv_message(mut stub: ServerStreamingClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let request = Number { value: 5 };
    
    let mut response_stream = stub
        .get_server_response(tonic::Request::new(request))
        .await?
        .into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("[server to client] {}", response.message);
    }

    Ok(())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051")
        .connect()
        .await?;

    let stub = ServerStreamingClient::new(channel);

    recv_message(stub).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}
