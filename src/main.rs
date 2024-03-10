mod client;
mod codes;
mod server;
mod transfer;
#[tokio::main]
async fn main() {
    server::main().await;
    println!("Hello, world!");
}
