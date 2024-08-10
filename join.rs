use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

async fn job(n: u64) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("{n}");
}

struct JoinAll<F> {
    futures: Vec<Pin<Box<F>>>,
}

impl<F: Future> Future for JoinAll<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        let is_pending = |f: &mut Pin<Box<F>>| f.as_mut().poll(context).is_pending();
        self.futures.retain_mut(is_pending);
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

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=20_000 {
        futures.push(job(n));
    }
    join_all(futures).await;
}
