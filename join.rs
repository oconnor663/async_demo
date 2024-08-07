use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

async fn work(name: &str, seconds: f32) {
    let duration = Duration::from_secs_f32(seconds);
    println!("{name}: start");
    tokio::time::sleep(duration / 2).await;
    println!("{name}: middle");
    tokio::time::sleep(duration / 2).await;
    println!("{name}: end");
}

struct JoinFuture<F1, F2> {
    f1: Pin<Box<F1>>,
    f1_ready: bool,
    f2: Pin<Box<F2>>,
    f2_ready: bool,
}

impl<F1: Future, F2: Future> Future for JoinFuture<F1, F2> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if !self.f1_ready {
            self.f1_ready = self.f1.as_mut().poll(context).is_ready();
        }
        if !self.f2_ready {
            self.f2_ready = self.f2.as_mut().poll(context).is_ready();
        }
        if self.f1_ready && self.f2_ready {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn join<F1, F2>(f1: F1, f2: F2) -> JoinFuture<F1, F2> {
    JoinFuture {
        f1: Box::pin(f1),
        f1_ready: false,
        f2: Box::pin(f2),
        f2_ready: false,
    }
}

#[tokio::main]
async fn main() {
    join(work("foo", 1.5), work("bar", 2.0)).await;
}
