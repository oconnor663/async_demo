use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

async fn job(n: u64) {
    tokio::time::sleep(Duration::from_millis(n)).await;
    println!("{n}");
}

struct Timeout<F> {
    sleep: Pin<Box<tokio::time::Sleep>>,
    inner: Pin<Box<F>>,
}

impl<F: Future> Future for Timeout<F> {
    type Output = Option<F::Output>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        if self.sleep.as_mut().poll(context).is_ready() {
            Poll::Ready(None)
        } else {
            match self.inner.as_mut().poll(context) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(output) => Poll::Ready(Some(output)),
            }
        }
    }
}

fn timeout<F: Future>(inner: F, duration: Duration) -> Timeout<F> {
    Timeout {
        sleep: Box::pin(tokio::time::sleep(duration)),
        inner: Box::pin(inner),
    }
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=20_000 {
        futures.push(job(n));
    }
    let all = future::join_all(futures);
    timeout(all, Duration::from_secs(1)).await;
}
