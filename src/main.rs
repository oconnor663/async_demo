use futures::future;
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll};
use std::thread;
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
            let sleep_duration = self.end_time - now;
            let main_thread = thread::current();
            thread::spawn(move || {
                thread::sleep(sleep_duration);
                main_thread.unpark();
            });
            Poll::Pending
        }
    }
}

fn async_sleep(seconds: f32) -> SleepFuture {
    let end_time = Instant::now() + Duration::from_secs_f32(seconds);
    SleepFuture { end_time }
}

async fn foo(message: &str) {
    async_sleep(1.0).await;
    println!("{message}");
}

async fn async_main() {
    foo("one").await;
    future::join(foo("two"), foo("three")).await;
}

fn main() {
    let mut main_future = pin!(async_main());
    let waker = noop_waker::noop_waker();
    let mut context = Context::from_waker(&waker);
    while let Poll::Pending = main_future.as_mut().poll(&mut context) {
        thread::park();
    }
}
