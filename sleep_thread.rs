use futures::future;
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

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

async fn work(name: &str, seconds: f32) {
    let duration = Duration::from_secs_f32(seconds);
    println!("{name}: start");
    sleep(duration / 2).await;
    println!("{name}: middle");
    sleep(duration / 2).await;
    println!("{name}: end");
}

#[tokio::main]
async fn main() {
    future::join(work("foo", 1.5), work("bar", 2.0)).await;
}
