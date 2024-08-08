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
    let future1 = work();
    let future2 = work();
    let future3 = work();
    future::join_all([future1, future2, future3]).await;
    println!();
}
