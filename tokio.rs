use std::time::Duration;

async fn foo() {
    println!("foo start");
    tokio::time::sleep(Duration::from_secs_f64(2.5)).await;
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    tokio::time::sleep(Duration::from_secs_f64(2.0)).await;
    println!("bar end");
}

#[tokio::main]
async fn main() {
    futures::future::join(foo(), bar()).await;
}
