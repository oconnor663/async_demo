use futures::future;
use std::io::Write;
use std::time::Duration;

async fn work() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    print!(".");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    future::join3(work(), work(), work()).await;
    println!();
}
