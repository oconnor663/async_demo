use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct JobFuture {
    sleep_future: Pin<Box<tokio::time::Sleep>>,
    n: u64,
}

impl Future for JobFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.sleep_future.as_mut().poll(context).is_pending() {
            Poll::Pending
        } else {
            println!("{}", self.n);
            Poll::Ready(())
        }
    }
}

fn job(n: u64) -> JobFuture {
    let sleep_future = tokio::time::sleep(Duration::from_secs(1));
    JobFuture {
        sleep_future: Box::pin(sleep_future),
        n,
    }
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=1_000 {
        futures.push(job(n));
    }
    future::join_all(futures).await;
}
