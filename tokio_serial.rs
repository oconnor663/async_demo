use std::io::Write;
use std::time::Duration;

async fn work(n: u64) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    print!("{n} ");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    work(1).await;
    work(2).await;
    work(3).await;
    println!();
}
