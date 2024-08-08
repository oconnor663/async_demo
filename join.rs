use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

static X: AtomicU64 = AtomicU64::new(0);

async fn work() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    X.fetch_add(1, Relaxed);
}

struct Join3<F1, F2, F3> {
    f1: Pin<Box<F1>>,
    f1_pending: bool,
    f2: Pin<Box<F2>>,
    f2_pending: bool,
    f3: Pin<Box<F3>>,
    f3_pending: bool,
}

impl<F1: Future, F2: Future, F3: Future> Future for Join3<F1, F2, F3> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.f1_pending {
            self.f1_pending = self.f1.as_mut().poll(context).is_pending();
        }
        if self.f2_pending {
            self.f2_pending = self.f2.as_mut().poll(context).is_pending();
        }
        if self.f3_pending {
            self.f3_pending = self.f3.as_mut().poll(context).is_pending();
        }
        if self.f1_pending || self.f2_pending || self.f3_pending {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

fn join3<F1: Future, F2: Future, F3: Future>(f1: F1, f2: F2, f3: F3) -> Join3<F1, F2, F3> {
    Join3 {
        f1: Box::pin(f1),
        f1_pending: true,
        f2: Box::pin(f2),
        f2_pending: true,
        f3: Box::pin(f3),
        f3_pending: true,
    }
}

async fn lots_of_work() {
    join3(work(), work(), work()).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    lots_of_work().await;
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
