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

async fn async_main() {
    futures::future::join(foo(), bar()).await;
}

fn main() {
    let mut main_future = Box::pin(async_main());
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
