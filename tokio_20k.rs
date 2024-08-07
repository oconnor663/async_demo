use futures::future;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Duration;

static X: AtomicU64 = AtomicU64::new(0);

async fn work() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    X.fetch_add(1, Relaxed);
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    future::join_all(futures).await;
    println!("X: {}", X.load(Relaxed));
}
