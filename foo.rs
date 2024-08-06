use futures::task::noop_waker_ref;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

static NEXT_WAKE_TIME: Mutex<Option<Instant>> = Mutex::new(None);

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
            let mut next_wake_time = NEXT_WAKE_TIME.lock().unwrap();
            if next_wake_time.is_none() || self.wake_time < next_wake_time.unwrap() {
                *next_wake_time = Some(self.wake_time);
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

fn main() {
    let mut main_future = Box::pin(futures::future::join(foo(), bar()));
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        let mut next_wake_time = NEXT_WAKE_TIME.lock().unwrap();
        let sleep_duration = next_wake_time
            .expect("somebody should want a wakeup")
            .checked_duration_since(Instant::now());
        if let Some(duration) = sleep_duration {
            *next_wake_time = None;
            std::thread::sleep(duration);
        }
    }
}
