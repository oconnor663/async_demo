use std::io::Write;
use std::time::Duration;

async fn work() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    print!(".");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    work().await;
    work().await;
    work().await;
    println!();
}
