use crate::codes::decode::ResponseDecode;
use tokio::net::TcpListener;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // start listening on 42.192.75.76:7700
    let listener = TcpListener::bind("0.0.0.0:7700").await?;
    //println!("echo server started!");
    let mut count: u32 = 1;
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            count += 1;
            tokio::spawn(async move {
                let mut response = ResponseDecode {
                    stream,
                    req_msg: None,
                };
                response.rece().await;
                println!("count {:}", count);
            });
        } else {
            continue;
        }
    }
}
