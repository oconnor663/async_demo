use std::future::Future;
use std::task::Poll;
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

fn join(f1: impl Future, f2: impl Future) -> impl Future {
    let mut f1 = Box::pin(f1);
    let mut f2 = Box::pin(f2);
    let mut f1_pending = true;
    let mut f2_pending = true;
    std::future::poll_fn(move |context| {
        if f1_pending {
            f1_pending = f1.as_mut().poll(context).is_pending();
        }
        if f2_pending {
            f2_pending = f2.as_mut().poll(context).is_pending();
        }
        if f1_pending || f2_pending {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
}

#[tokio::main]
async fn main() {
    join(foo(), bar()).await;
}
