use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct SleepFuture {
    wake_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        let now = Instant::now();
        if now >= self.wake_time {
            Poll::Ready(())
        } else {
            let time_remaining = self.wake_time - now;
            let waker = context.waker().clone();
            std::thread::spawn(move || {
                std::thread::sleep(time_remaining);
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

static X: AtomicU64 = AtomicU64::new(0);

async fn work() {
    sleep(Duration::from_secs(1)).await;
    X.fetch_add(1, Relaxed);
}

async fn lots_of_work() {
    future::join3(work(), work(), work()).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    lots_of_work().await;
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
