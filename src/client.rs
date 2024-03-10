use crate::transfer::transfer::RequestTran;
use tokio::net::TcpStream;

use crate::codes::encode::RequestEncode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a peer

    for i in 1..80 {
        let stream: TcpStream = TcpStream::connect("127.0.0.1:8080").await?;
        let mut transfer: RequestTran = RequestTran::new();
        transfer.amount = 1;
        transfer.to = "test".to_string();
        transfer.from = "test".to_string();
        let mut encode: RequestEncode = RequestEncode { stream, transfer };
        encode.send().await;
    }
    Ok(())
}
