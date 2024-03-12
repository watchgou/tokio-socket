use crate::my_request::request_1::RequestMessage;
use tokio::net::TcpStream;

use crate::codes::encode::RequestEncode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a peer

    for i in 1..80 {
        let stream: TcpStream = TcpStream::connect("127.0.0.1:8080").await?;
        let mut req_msg: RequestMessage = RequestMessage::new();
        req_msg.code=1;
        req_msg.message="success".to_string();
        let mut encode: RequestEncode = RequestEncode { stream, req_msg };
        encode.send().await;
    }
    Ok(())
}
