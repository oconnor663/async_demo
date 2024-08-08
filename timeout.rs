use futures::future;
use rand::prelude::*;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

async fn work() {
    let mut rng = rand::thread_rng();
    let seconds = rng.gen_range(0.0..1.0);
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
    print!(".");
    std::io::stdout().flush().unwrap();
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
    for _ in 0..20_000 {
        futures.push(work());
    }
    let all = future::join_all(futures);
    timeout(all, Duration::from_secs_f32(0.5)).await;
    println!();
}
