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
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    future::join_all(futures).await;
    println!();
}
