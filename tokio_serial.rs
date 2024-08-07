use std::time::Duration;

async fn work(name: &str, seconds: f32) {
    let duration = Duration::from_secs_f32(seconds);
    println!("{name}: start");
    tokio::time::sleep(duration / 2).await;
    println!("{name}: middle");
    tokio::time::sleep(duration / 2).await;
    println!("{name}: end");
}

#[tokio::main]
async fn main() {
    work("foo", 1.5).await;
    work("bar", 2.0).await;
}
