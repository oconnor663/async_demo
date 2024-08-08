use rand::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

static X: AtomicU64 = AtomicU64::new(0);

async fn work() {
    let mut rng = rand::thread_rng();
    let seconds = rng.gen_range(0.0..1.0);
    println!("work time: {:.3} seconds", seconds);
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
    X.fetch_add(1, Relaxed);
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
    let start = Instant::now();
    timeout(work(), Duration::from_secs_f32(0.5)).await;
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
