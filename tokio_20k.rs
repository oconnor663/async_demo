use futures::future;
use std::io::Write;
use std::time::Duration;

async fn work(n: u64) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    print!("{n} ");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=20_000 {
        futures.push(work(n));
    }
    future::join_all(futures).await;
    println!();
}
