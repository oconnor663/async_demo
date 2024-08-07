use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct WorkFuture {
    name: String,
    duration: Duration,
    state: WorkState,
}

enum WorkState {
    Start,
    FirstSleep(Pin<Box<tokio::time::Sleep>>),
    SecondSleep(Pin<Box<tokio::time::Sleep>>),
    End,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if let WorkState::Start = self.state {
            println!("{} start", self.name);
            let sleep_future = tokio::time::sleep(self.duration / 2);
            self.state = WorkState::FirstSleep(Box::pin(sleep_future));
        }
        if let WorkState::FirstSleep(sleep_future) = &mut self.state {
            if sleep_future.as_mut().poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("{} middle", self.name);
            let sleep_future = tokio::time::sleep(self.duration / 2);
            self.state = WorkState::SecondSleep(Box::pin(sleep_future));
        }
        if let WorkState::SecondSleep(sleep_future) = &mut self.state {
            if sleep_future.as_mut().poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("{} end", self.name);
            self.state = WorkState::End;
            return Poll::Ready(());
        }
        unreachable!("polled again after Ready");
    }
}

fn work(name: &str, seconds: f32) -> WorkFuture {
    WorkFuture {
        name: String::from(name),
        duration: Duration::from_secs_f32(seconds),
        state: WorkState::Start,
    }
}

#[tokio::main]
async fn main() {
    future::join(work("foo", 1.5), work("bar", 2.0)).await;
}
