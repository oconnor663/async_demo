use futures::future;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

struct SleepFuture {
    end_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        let now = Instant::now();
        if now >= self.end_time {
            Poll::Ready(())
        } else {
            let sleep_duration = self.end_time - now;
            let waker = context.waker().clone();
            thread::spawn(move || {
                thread::sleep(sleep_duration);
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn sleep(seconds: f32) -> SleepFuture {
    let end_time = Instant::now() + Duration::from_secs_f32(seconds);
    SleepFuture { end_time }
}

async fn foo(message: &str) {
    sleep(1.0).await;
    println!("{message}");
}

async fn async_main() {
    foo("one").await;
    future::join(foo("two"), foo("three")).await;
}

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn main() {
    let mut main_future = pin!(async_main());
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut context = Context::from_waker(&waker);
    while let Poll::Pending = main_future.as_mut().poll(&mut context) {
        thread::park();
    }
}
