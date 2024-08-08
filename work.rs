use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

static X: AtomicU64 = AtomicU64::new(0);

struct WorkFuture {
    sleep_future: Pin<Box<tokio::time::Sleep>>,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.sleep_future.as_mut().poll(context).is_pending() {
            Poll::Pending
        } else {
            X.fetch_add(1, Relaxed);
            Poll::Ready(())
        }
    }
}

fn work() -> WorkFuture {
    let sleep_future = tokio::time::sleep(Duration::from_secs(1));
    WorkFuture {
        sleep_future: Box::pin(sleep_future),
    }
}

async fn lots_of_work() {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    future::join_all(futures).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    lots_of_work().await;
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
