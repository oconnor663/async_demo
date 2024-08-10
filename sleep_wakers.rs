use futures::future;
use futures::task::noop_waker_ref;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

std::thread_local! {
    static WAKERS: RefCell<BTreeMap<Instant, Vec<Waker>>> = RefCell::new(BTreeMap::new());
}

struct SleepFuture {
    wake_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.wake_time <= Instant::now() {
            Poll::Ready(())
        } else {
            WAKERS.with_borrow_mut(|tree| {
                tree.entry(self.wake_time)
                    .or_default()
                    .push(context.waker().clone());
            });
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

async fn job(n: u64) {
    sleep(Duration::from_secs(1)).await;
    println!("{n}");
}

fn main() {
    let mut futures = Vec::new();
    for n in 1..=20_000 {
        futures.push(job(n));
    }
    let mut main_future = Box::pin(future::join_all(futures));
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        WAKERS.with_borrow_mut(|tree| {
            let (first_wake_time, _) = tree
                .first_key_value()
                .expect("poll returned Pending, so there must be a Waker");
            std::thread::sleep(first_wake_time.saturating_duration_since(Instant::now()));
            while let Some((&wake_time, wakers)) = tree.first_key_value() {
                if wake_time <= Instant::now() {
                    wakers.iter().for_each(Waker::wake_by_ref);
                    tree.pop_first();
                } else {
                    break;
                }
            }
        });
    }
}
