use futures::future;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::{Duration, Instant};

static X: AtomicU64 = AtomicU64::new(0);

async fn work() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    X.fetch_add(1, Relaxed);
}

async fn lots_of_work() {
    future::join3(work(), work(), work()).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    lots_of_work().await;
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
