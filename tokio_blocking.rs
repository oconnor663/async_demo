use futures::future;
use std::time::Duration;

async fn job(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=20_000 {
        futures.push(job(n));
    }
    future::join_all(futures).await;
}
