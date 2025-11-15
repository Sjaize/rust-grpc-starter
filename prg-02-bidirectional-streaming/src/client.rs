// Bidirectional streaming gRPC client

use tonic::transport::Channel;

use prg_02_bidirectional_streaming::bidirectional_pb2_grpc::bidirectional_client::BidirectionalClient;
use prg_02_bidirectional_streaming::bidirectional_pb2::Message;

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

async fn send_message(mut stub: BidirectionalClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let messages: Vec<Message> = generate_messages().collect();
    let request_stream = async_stream::stream! {
        for msg in messages {
            println!("[client to server] {}", msg.message);
            yield msg;
        }
    };

    let mut response_stream = stub
        .get_server_response(tonic::Request::new(request_stream))
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

    let stub = BidirectionalClient::new(channel);

    send_message(stub).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}
