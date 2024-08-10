use std::time::Duration;

async fn work(n: u64) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("{n}");
}

#[tokio::main]
async fn main() {
    let mut tasks = Vec::new();
    for n in 1..=20_000 {
        tasks.push(tokio::task::spawn(work(n)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
