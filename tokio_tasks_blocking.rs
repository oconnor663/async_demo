use std::time::Duration;

async fn job(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

#[tokio::main]
async fn main() {
    let mut tasks = Vec::new();
    for n in 1..=1_000 {
        tasks.push(tokio::task::spawn(job(n)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
