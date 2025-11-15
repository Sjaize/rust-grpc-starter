// Bidirectional streaming gRPC server

use tonic::transport::Server;
use futures::Stream;
use std::pin::Pin;

use prg_02_bidirectional_streaming::bidirectional_pb2_grpc::bidirectional_server::{Bidirectional, BidirectionalServer};
use prg_02_bidirectional_streaming::bidirectional_pb2::Message;

struct BidirectionalService;

#[tonic::async_trait]
impl Bidirectional for BidirectionalService {
    type GetServerResponseStream = Pin<Box<dyn Stream<Item = Result<Message, tonic::Status>> + Send>>;

    async fn get_server_response(
        &self,
        request: tonic::Request<tonic::Streaming<Message>>,
    ) -> Result<tonic::Response<Self::GetServerResponseStream>, tonic::Status> {
        println!("Server processing gRPC bidirectional streaming.");
        
        let mut input_stream = request.into_inner();
        let output_stream = async_stream::stream! {
            while let Ok(Some(message)) = input_stream.message().await {
                yield Ok(message);
            }
        };

        Ok(tonic::Response::new(Box::pin(output_stream)))
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::builder();
    let server = server.add_service(BidirectionalServer::new(BidirectionalService));
    
    println!("Starting server. Listening on port 50051.");
    let addr = "[::]:50051".parse()?;
    
    server.serve(addr).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    serve().await
}
