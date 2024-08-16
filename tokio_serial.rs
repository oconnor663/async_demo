use std::time::Duration;

async fn job(n: u64) {
    println!("start {n}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("end {n}");
}

#[tokio::main]
async fn main() {
    job(1).await;
    job(2).await;
    job(3).await;
}
