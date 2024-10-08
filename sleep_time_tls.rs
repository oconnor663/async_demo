use futures::future;
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
        if self.wake_time <= Instant::now() {
            Poll::Ready(())
        } else {
            let next = NEXT_WAKE_TIME.get();
            if next.is_none() || self.wake_time < next.unwrap() {
                NEXT_WAKE_TIME.set(Some(self.wake_time));
            }
            // OOPS: We're returning Pending without ever calling wake(). See below.
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

async fn job(n: u64) {
    println!("start {n}");
    sleep(Duration::from_secs(1)).await;
    println!("end {n}");
}

fn main() {
    let mut futures = Vec::new();
    // OOPS: Because we never call wake() above, this works for 30 futures but not 31!
    // https://docs.rs/futures/0.3.30/futures/future/fn.join_all.html#see-also
    for n in 1..=30 {
        futures.push(job(n));
    }
    let mut main_future = Box::pin(future::join_all(futures));
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        dbg!(NEXT_WAKE_TIME.get().is_some());
        let next = NEXT_WAKE_TIME
            .get()
            .expect("OOPS! JoinAll won't poll our sleeps again if they don't wake().");
        let sleep_duration = next.saturating_duration_since(Instant::now());
        dbg!(sleep_duration);
        NEXT_WAKE_TIME.set(None);
        std::thread::sleep(sleep_duration);
    }
}
