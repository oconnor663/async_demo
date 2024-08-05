use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration};

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
    join(foo(), bar()).await;
}
