use futures::future;
use std::time::Duration;

async fn job(n: u64) {
    println!("start {n}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("end {n}");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=1_000 {
        futures.push(job(n));
    }
    future::join_all(futures).await;
}
