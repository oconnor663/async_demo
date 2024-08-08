use futures::future;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct WorkFuture {
    sleep_future: Pin<Box<tokio::time::Sleep>>,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.sleep_future.as_mut().poll(context).is_pending() {
            Poll::Pending
        } else {
            print!(".");
            std::io::stdout().flush().unwrap();
            Poll::Ready(())
        }
    }
}

fn work() -> WorkFuture {
    let sleep_future = tokio::time::sleep(Duration::from_secs(1));
    WorkFuture {
        sleep_future: Box::pin(sleep_future),
    }
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for _ in 0..20_000 {
        futures.push(work());
    }
    future::join_all(futures).await;
    println!();
}
