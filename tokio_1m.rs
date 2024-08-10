use futures::future;
use std::time::Duration;

async fn work(n: u64) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("{n}");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=1_000_000 {
        futures.push(work(n));
    }
    future::join_all(futures).await;
}
