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
    let t1 = tokio::task::spawn(work("foo", 1.5));
    let t2 = tokio::task::spawn(work("bar", 2.0));
    t1.await.unwrap();
    t2.await.unwrap();
}
