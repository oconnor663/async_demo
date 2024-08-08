use futures::future;
use futures::task::noop_waker_ref;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

std::thread_local! {
    static WAKERS: RefCell<BTreeMap<Instant, Waker>> = RefCell::new(BTreeMap::new());
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
                tree.insert(self.wake_time, context.waker().clone());
            });
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

async fn work() {
    sleep(Duration::from_secs(1)).await;
    print!(".");
    std::io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    let mut main_future = Box::pin(future::join_all(futures));
    let mut context = Context::from_waker(noop_waker_ref());
    while main_future.as_mut().poll(&mut context).is_pending() {
        WAKERS.with_borrow_mut(|tree| {
            while let Some((&wake_time, waker)) = tree.first_key_value() {
                let now = Instant::now();
                if wake_time <= now {
                    waker.wake_by_ref();
                    tree.pop_first();
                } else {
                    std::thread::sleep(wake_time.saturating_duration_since(now));
                }
            }
        });
    }
    println!();
}
