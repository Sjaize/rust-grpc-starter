// Bidirectional streaming gRPC server

use tonic::transport::Server;

use prg_03_clientstreaming::clientstreaming_pb2_grpc::client_streaming_server::{ClientStreaming, ClientStreamingServer};
use prg_03_clientstreaming::clientstreaming_pb2::{Message, Number};

struct ClientStreamingServicer;

#[tonic::async_trait]
impl ClientStreaming for ClientStreamingServicer {
    async fn get_server_response(
        &self,
        request: tonic::Request<tonic::Streaming<Message>>,
    ) -> Result<tonic::Response<Number>, tonic::Status> {
        println!("Server processing gRPC client-streaming.");
        
        let mut input_stream = request.into_inner();
        let mut count = 0;
        
        while let Ok(Some(_message)) = input_stream.message().await {
            count += 1;
        }
        
        Ok(tonic::Response::new(Number { value: count }))
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::builder()
        .add_service(ClientStreamingServer::new(ClientStreamingServicer));
    
    println!("Starting server. Listening on port 50051.");
    let addr = "[::]:50051".parse()?;
    
    server.serve(addr).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    serve().await
}
