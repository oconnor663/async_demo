use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

fn sleep(seconds: f64) -> tokio::time::Sleep {
    let duration = Duration::from_secs_f64(seconds);
    tokio::time::sleep(duration)
}

enum FooFuture {
    Start,
    FirstSleep(Pin<Box<tokio::time::Sleep>>),
    SecondSleep(Pin<Box<tokio::time::Sleep>>),
    End,
}

impl Future for FooFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if let FooFuture::Start = *self {
            println!("foo start");
            *self = FooFuture::FirstSleep(Box::pin(sleep(0.5)));
        }
        if let FooFuture::FirstSleep(sleep_future) = &mut *self {
            if Pin::new(sleep_future).poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("foo middle");
            *self = FooFuture::SecondSleep(Box::pin(sleep(1.0)));
        }
        if let FooFuture::SecondSleep(sleep_future) = &mut *self {
            if Pin::new(sleep_future).poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("foo end");
            *self = FooFuture::End;
            return Poll::Ready(());
        }
        unreachable!("polled again after Ready");
    }
}

fn foo() -> FooFuture {
    FooFuture::Start
}

async fn bar() {
    println!("bar start");
    sleep(1.0).await;
    println!("bar middle");
    sleep(1.0).await;
    println!("bar end");
}

#[tokio::main]
async fn main() {
    futures::future::join(foo(), bar()).await;
}
