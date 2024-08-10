use std::time::Duration;

async fn work(n: u64) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("{n}");
}

#[tokio::main]
async fn main() {
    work(1).await;
    work(2).await;
    work(3).await;
}
