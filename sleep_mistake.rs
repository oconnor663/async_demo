use std::time::Duration;

async fn foo() {
    println!("foo start");
    std::thread::sleep(Duration::from_secs_f64(2.5));
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    std::thread::sleep(Duration::from_secs_f64(2.0));
    println!("bar end");
}

#[tokio::main]
async fn main() {
    futures::future::join(foo(), bar()).await;
}
