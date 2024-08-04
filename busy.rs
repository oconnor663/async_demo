use futures::task::noop_waker_ref;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct SleepFuture {
    wake_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<()> {
        let now = Instant::now();
        if now >= self.wake_time {
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

async fn foo() {
    println!("foo start");
    sleep(Duration::from_secs_f64(2.5)).await;
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    sleep(Duration::from_secs_f64(2.0)).await;
    println!("bar end");
}

fn join<F1, F2>(f1: F1, f2: F2) -> JoinFuture<F1, F2> {
    JoinFuture {
        f1: Box::pin(f1),
        f1_ready: false,
        f2: Box::pin(f2),
        f2_ready: false,
    }
}

struct JoinFuture<F1, F2> {
    f1: Pin<Box<F1>>,
    f1_ready: bool,
    f2: Pin<Box<F2>>,
    f2_ready: bool,
}

impl<F1: Future, F2: Future> Future for JoinFuture<F1, F2> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if !self.f1_ready {
            self.f1_ready = self.f1.as_mut().poll(context).is_ready();
        }
        if !self.f2_ready {
            self.f2_ready = self.f2.as_mut().poll(context).is_ready();
        }
        if self.f1_ready && self.f2_ready {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

async fn async_main() {
    join(foo(), bar()).await;
}

fn main() {
    let mut main_future = Box::pin(async_main());
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        // busy loop!
    }
}
