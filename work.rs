use futures::future;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct WorkFuture {
    n: u64,
    sleep_future: Pin<Box<tokio::time::Sleep>>,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.sleep_future.as_mut().poll(context).is_pending() {
            Poll::Pending
        } else {
            print!("{} ", self.n);
            std::io::stdout().flush().unwrap();
            Poll::Ready(())
        }
    }
}

fn work(n: u64) -> WorkFuture {
    let sleep_future = Box::pin(tokio::time::sleep(Duration::from_secs(1)));
    WorkFuture { n, sleep_future }
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=20_000 {
        futures.push(work(n));
    }
    future::join_all(futures).await;
    println!();
}
