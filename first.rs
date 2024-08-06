use futures::task::noop_waker_ref;
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

std::thread_local! {
    static NEXT_WAKE_TIME: Cell<Option<Instant>> = Cell::new(None);
}

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
            let next = NEXT_WAKE_TIME.get();
            if next.is_none() || self.wake_time < next.unwrap() {
                NEXT_WAKE_TIME.set(Some(self.wake_time));
            }
            Poll::Pending
        }
    }
}

fn sleep(seconds: f64) -> SleepFuture {
    let duration = Duration::from_secs_f64(seconds);
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

enum FooFuture {
    Start,
    FirstSleep(SleepFuture),
    SecondSleep(SleepFuture),
    End,
}

impl Future for FooFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if let FooFuture::Start = *self {
            println!("foo start");
            *self = FooFuture::FirstSleep(sleep(0.5));
        }
        if let FooFuture::FirstSleep(sleep_future) = &mut *self {
            if Pin::new(sleep_future).poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("foo middle");
            *self = FooFuture::SecondSleep(sleep(1.0));
        }
        if let FooFuture::SecondSleep(sleep_future) = &mut *self {
            if Pin::new(sleep_future).poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("foo end");
            *self = FooFuture::End;
            return Poll::Ready(());
        }
        unreachable!("polled again after Ready");
    }
}

fn foo() -> FooFuture {
    FooFuture::Start
}

async fn bar() {
    println!("bar start");
    sleep(1.0).await;
    println!("bar middle");
    sleep(1.0).await;
    println!("bar end");
}

struct FirstFuture<F1, F2> {
    f1: Pin<Box<F1>>,
    f1_ready: bool,
    f2: Pin<Box<F2>>,
    f2_ready: bool,
}

impl<F1: Future, F2: Future> Future for FirstFuture<F1, F2> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if !self.f1_ready {
            self.f1_ready = self.f1.as_mut().poll(context).is_ready();
        }
        if !self.f2_ready {
            self.f2_ready = self.f2.as_mut().poll(context).is_ready();
        }
        if self.f1_ready || self.f2_ready {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn first<F1, F2>(f1: F1, f2: F2) -> FirstFuture<F1, F2> {
    FirstFuture {
        f1: Box::pin(f1),
        f1_ready: false,
        f2: Box::pin(f2),
        f2_ready: false,
    }
}

fn main() {
    let mut main_future = Box::pin(first(foo(), bar()));
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        let next = NEXT_WAKE_TIME.get().expect("someone must want a wakeup");
        let duration = next.saturating_duration_since(Instant::now());
        NEXT_WAKE_TIME.set(None);
        std::thread::sleep(duration);
    }
}
