use std::time::Duration;

async fn sleep(seconds: f64) {
    let duration = Duration::from_secs_f64(seconds);
    std::thread::sleep(duration);
}

async fn foo() {
    println!("foo start");
    sleep(0.5).await;
    println!("foo middle");
    sleep(1.0).await;
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    sleep(1.0).await;
    println!("bar middle");
    sleep(1.0).await;
    println!("bar end");
}

#[tokio::main]
async fn main() {
    futures::future::join(foo(), bar()).await;
}
