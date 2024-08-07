use std::future::Future;
use std::pin::Pin;
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

fn sleep(seconds: f64) -> SleepFuture {
    let duration = Duration::from_secs_f64(seconds);
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

async fn foo() {
    println!("foo start");
    sleep(0.5).await;
    println!("foo middle");
    sleep(1.0).await;
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    sleep(1.0).await;
    println!("bar middle");
    sleep(1.0).await;
    println!("bar end");
}

#[tokio::main]
async fn main() {
    futures::future::join(foo(), bar()).await;
}
