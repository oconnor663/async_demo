use futures::future;
use futures::task::noop_waker_ref;
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct SleepFuture {
    end_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<()> {
        let now = Instant::now();
        if now >= self.end_time {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn async_sleep(seconds: f32) -> SleepFuture {
    let end_time = Instant::now() + Duration::from_secs_f32(seconds);
    SleepFuture { end_time }
}

async fn foo(message: &str) {
    async_sleep(3.0).await;
    println!("{message}");
}

async fn async_main() {
    foo("one").await;
    future::join(foo("two"), foo("three")).await;
}

fn main() {
    let mut main_future = pin!(async_main());
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {}
}
