use crate::codes::decode::ResponseDecode;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // start listening on 50007
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("echo server started!");

    loop {
        let (mut stream, addr) = listener.accept().await?;

        println!("accepted connection from: {}", addr);
        tokio::spawn(async move {
            let mut response = ResponseDecode {
                stream,
                transfer: None,
            };
            response.rece().await;
        });
    }
}
