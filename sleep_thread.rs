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

struct WorkFuture {
    sleep_future: Pin<Box<SleepFuture>>,
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
    let sleep_future = sleep(Duration::from_secs(1));
    WorkFuture {
        sleep_future: Box::pin(sleep_future),
    }
}

struct JoinAll<F> {
    futures: Vec<Pin<Box<F>>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        self.futures
            .retain_mut(|future| future.as_mut().poll(context).is_pending());
        if self.futures.is_empty() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    JoinAll {
        futures: futures.into_iter().map(Box::pin).collect(),
    }
}

fn lots_of_work() -> JoinAll<WorkFuture> {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    join_all(futures)
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    lots_of_work().await;
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
