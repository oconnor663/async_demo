use futures::task::noop_waker_ref;
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::{thread, thread_local};

thread_local! {
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

fn async_sleep(seconds: f64) -> SleepFuture {
    let wake_time = Instant::now() + Duration::from_secs_f64(seconds);
    SleepFuture { wake_time }
}

async fn foo() {
    println!("foo start");
    async_sleep(2.0).await;
    println!("foo end");
}

async fn bar() {
    println!("bar start");
    async_sleep(2.5).await;
    println!("bar end");
}

fn join(f1: impl Future, f2: impl Future) -> impl Future {
    let mut f1 = Box::pin(f1);
    let mut f2 = Box::pin(f2);
    let mut f1_pending = true;
    let mut f2_pending = true;
    std::future::poll_fn(move |context| {
        if f1_pending {
            f1_pending = f1.as_mut().poll(context).is_pending();
        }
        if f2_pending {
            f2_pending = f2.as_mut().poll(context).is_pending();
        }
        if f1_pending || f2_pending {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
}

async fn async_main() {
    join(foo(), bar()).await;
}

fn main() {
    let mut main_future = Box::pin(async_main());
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        let next = NEXT_WAKE_TIME.get().expect("somebody better wake us up");
        if let Some(sleep_duration) = next.checked_duration_since(Instant::now()) {
            NEXT_WAKE_TIME.set(None);
            thread::sleep(sleep_duration);
        }
    }
}
