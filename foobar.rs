use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

fn foo() -> StartEndFuture {
    StartEndFuture::new("foo", Duration::from_secs_f64(2.5))
}

fn bar() -> StartEndFuture {
    StartEndFuture::new("bar", Duration::from_secs_f64(2.0))
}

struct StartEndFuture {
    name: String,
    sleep_future: Pin<Box<tokio::time::Sleep>>,
    is_start: bool,
}

impl StartEndFuture {
    fn new(name: &str, duration: Duration) -> Self {
        Self {
            name: String::from(name),
            sleep_future: Box::pin(tokio::time::sleep(duration)),
            is_start: true,
        }
    }
}

impl Future for StartEndFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.is_start {
            println!("{} start", self.name);
            self.is_start = false;
        }
        if self.sleep_future.as_mut().poll(context).is_pending() {
            return Poll::Pending;
        }
        println!("{} end", self.name);
        Poll::Ready(())
    }
}

#[tokio::main]
async fn main() {
    futures::future::join(foo(), bar()).await;
}
