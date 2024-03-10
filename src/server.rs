use crate::codes::decode::ResponseDecode;
use tokio::net::TcpListener;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // start listening on 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    //println!("echo server started!");
    let mut count: u32 = 1;
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            count += 1;
            println!("count {:}", count);
            tokio::spawn(async move {
                let mut response = ResponseDecode {
                    stream,
                    transfer: None,
                };
                response.rece().await;
            });
        } else {
            continue;
        }
    }
}
