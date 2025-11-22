// Bidirectional streaming gRPC client

use tonic::transport::Channel;

use prg_03_clientstreaming::clientstreaming_pb2_grpc::client_streaming_client::ClientStreamingClient;
use prg_03_clientstreaming::clientstreaming_pb2::Message;

fn make_message(message: &str) -> Message {
    Message {
        message: message.to_string(),
    }
}

fn generate_messages() -> impl Iterator<Item = Message> {
    let messages = vec![
        make_message("message #1"),
        make_message("message #2"),
        make_message("message #3"),
        make_message("message #4"),
        make_message("message #5"),
    ];
    messages.into_iter()
}

async fn send_message(mut stub: ClientStreamingClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let messages: Vec<Message> = generate_messages().collect();
    let request_stream = async_stream::stream! {
        for msg in messages {
            println!("[client to server] {}", msg.message);
            yield msg;
        }
    };

    let response = stub
        .get_server_response(tonic::Request::new(Box::pin(request_stream)))
        .await?;

    println!("[server to client] {}", response.get_ref().value);

    Ok(())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051")
        .connect()
        .await?;

    let stub = ClientStreamingClient::new(channel);

    send_message(stub).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}
