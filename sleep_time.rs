use futures::future;
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
        if self.wake_time <= Instant::now() {
            Poll::Ready(())
        } else {
            let mut next = NEXT_WAKE_TIME.lock().unwrap();
            if next.is_none() || self.wake_time < next.unwrap() {
                *next = Some(self.wake_time);
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
    // NOTE: This example actually works if you reduce the number of jobs to 30 or less. As of
    // futures v0.3.30, that's the threshold for JoinAll to use FuturesOrdered internally. See:
    // https://docs.rs/futures/0.3.30/futures/future/fn.join_all.html#see-also
    for n in 1..=1_000 {
        futures.push(job(n));
    }
    let mut main_future = Box::pin(future::join_all(futures));
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        let mut next = NEXT_WAKE_TIME.lock().unwrap();
        dbg!(next.is_some());
        let sleep_duration = next
            .expect("OOPS! JoinAll won't poll our sleeps again if they don't wake().")
            .saturating_duration_since(Instant::now());
        dbg!(sleep_duration);
        *next = None;
        std::thread::sleep(sleep_duration);
    }
}
