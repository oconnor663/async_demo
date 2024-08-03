use futures::future::join;
use tokio::time::{sleep, Duration};

async fn foo() {
    println!("foo start");
    sleep(Duration::from_secs_f64(2.5)).await;
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    sleep(Duration::from_secs_f64(2.0)).await;
    println!("bar end");
}

#[tokio::main]
async fn main() {
    join(foo(), bar()).await;
}
