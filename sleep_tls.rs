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
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

static X: AtomicU64 = AtomicU64::new(0);

struct WorkFuture {
    sleep_future: Pin<Box<SleepFuture>>,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.sleep_future.as_mut().poll(context).is_pending() {
            Poll::Pending
        } else {
            X.fetch_add(1, Relaxed);
            Poll::Ready(())
        }
    }
}

fn work() -> WorkFuture {
    let sleep_future = sleep(Duration::from_secs(1));
    WorkFuture {
        sleep_future: Box::pin(sleep_future),
    }
}

struct JoinAll<F> {
    futures: Vec<Pin<Box<F>>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        self.futures
            .retain_mut(|future| future.as_mut().poll(context).is_pending());
        if self.futures.is_empty() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    JoinAll {
        futures: futures.into_iter().map(Box::pin).collect(),
    }
}

fn lots_of_work() -> JoinAll<WorkFuture> {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    join_all(futures)
}

fn block_on(future: impl Future) {
    let mut pinned = Box::pin(future);
    let mut context = Context::from_waker(noop_waker_ref());
    while pinned.as_mut().poll(&mut context).is_pending() {
        let next = NEXT_WAKE_TIME.get().expect("someone must want a wakeup");
        let duration = next.saturating_duration_since(Instant::now());
        NEXT_WAKE_TIME.set(None);
        std::thread::sleep(duration);
        // Note: This demo works with our simplified JoinAll above, but it wouldn't
        // work with futures::future::JoinAll, because we never call Waker::wake.
    }
}

fn main() {
    let start = Instant::now();
    block_on(lots_of_work());
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
