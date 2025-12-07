// Bidirectional streaming gRPC server

use tonic::transport::Server;
use futures::Stream;
use std::pin::Pin;

use prg_04_serverstreaming::serverstreaming_pb2_grpc::server_streaming_server::{ServerStreaming, ServerStreamingServer};
use prg_04_serverstreaming::serverstreaming_pb2::{Message, Number};

fn make_message(message: &str) -> Message {
    Message {
        message: message.to_string(),
    }
}

struct ServerStreamingService;

#[tonic::async_trait]
impl ServerStreaming for ServerStreamingService {
    type GetServerResponseStream = Pin<Box<dyn Stream<Item = Result<Message, tonic::Status>> + Send>>;

    async fn get_server_response(
        &self,
        request: tonic::Request<Number>,
    ) -> Result<tonic::Response<Self::GetServerResponseStream>, tonic::Status> {
        let request_value = request.into_inner().value;
        
        let messages = vec![
            make_message("message #1"),
            make_message("message #2"),
            make_message("message #3"),
            make_message("message #4"),
            make_message("message #5"),
        ];
        
        println!("Server processing gRPC server-streaming {{{}}}.", request_value);
        
        let output_stream = async_stream::stream! {
            for message in messages {
                yield Ok(message);
            }
        };

        Ok(tonic::Response::new(Box::pin(output_stream)))
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::builder()
        .add_service(ServerStreamingServer::new(ServerStreamingService));
    
    println!("Starting server. Listening on port 50051.");
    let addr = "[::]:50051".parse()?;
    
    server.serve(addr).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    serve().await
}
