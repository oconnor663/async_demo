use futures::future;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct SleepFuture {
    wake_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<()> {
        if self.wake_time <= Instant::now() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

async fn work() {
    sleep(Duration::from_secs(1)).await;
    print!(".");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    future::join_all(futures).await;
    println!();
}
