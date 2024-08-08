use futures::future;
use futures::task::noop_waker_ref;
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
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
            // Note: It's technically cheating to return Pending here without ever
            // calling wake. It works in this demo because we know we have a no-op
            // Context. But it doesn't work with combinators that substitute their
            // own Context, like JoinAll, FuturesOrdered, or FuturesUnordered.
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

static X: AtomicU64 = AtomicU64::new(0);

async fn work() {
    sleep(Duration::from_secs(1)).await;
    X.fetch_add(1, Relaxed);
}

async fn lots_of_work() {
    future::join3(work(), work(), work()).await;
}

fn block_on(future: impl Future) {
    let mut pinned = Box::pin(future);
    let mut context = Context::from_waker(noop_waker_ref());
    while pinned.as_mut().poll(&mut context).is_pending() {
        let next = NEXT_WAKE_TIME.get().expect("someone must want a wakeup");
        let duration = next.saturating_duration_since(Instant::now());
        NEXT_WAKE_TIME.set(None);
        std::thread::sleep(duration);
    }
}

fn main() {
    let start = Instant::now();
    block_on(lots_of_work());
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
