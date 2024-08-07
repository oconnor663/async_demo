use std::cell::Cell;
use std::task::Poll;
use std::time::{Duration, Instant};

trait Future {
    type Output;

    // Get rid of Pin and use an empty Context.
    fn poll(&mut self, context: &mut Context) -> Poll<()>;
}

struct Context {}

std::thread_local! {
    static NEXT_WAKE_TIME: Cell<Option<Instant>> = Cell::new(None);
}

struct SleepFuture {
    wake_time: Instant,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(&mut self, _: &mut Context) -> Poll<()> {
        let now = Instant::now();
        if now >= self.wake_time {
            Poll::Ready(())
        } else {
            let next = NEXT_WAKE_TIME.get();
            if next.is_none() || self.wake_time < next.unwrap() {
                NEXT_WAKE_TIME.set(Some(self.wake_time));
            }
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> SleepFuture {
    let wake_time = Instant::now() + duration;
    SleepFuture { wake_time }
}

struct WorkFuture {
    name: String,
    duration: Duration,
    state: WorkState,
}

enum WorkState {
    Start,
    FirstSleep(SleepFuture),
    SecondSleep(SleepFuture),
    End,
}

impl Future for WorkFuture {
    type Output = ();

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        if let WorkState::Start = self.state {
            println!("{} start", self.name);
            let sleep_future = sleep(self.duration / 2);
            self.state = WorkState::FirstSleep(sleep_future);
        }
        if let WorkState::FirstSleep(sleep_future) = &mut self.state {
            if sleep_future.poll(context).is_pending() {
                return Poll::Pending;
            }
            println!("{} middle", self.name);
            let sleep_future = sleep(self.duration / 2);
            self.state = WorkState::SecondSleep(sleep_future);
        }
        if let WorkState::SecondSleep(sleep_future) = &mut self.state {
            if sleep_future.poll(context).is_pending() {
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

struct JoinFuture<F1, F2> {
    f1: F1,
    f1_ready: bool,
    f2: F2,
    f2_ready: bool,
}

impl<F1: Future, F2: Future> Future for JoinFuture<F1, F2> {
    type Output = ();

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        if !self.f1_ready {
            self.f1_ready = self.f1.poll(context).is_ready();
        }
        if !self.f2_ready {
            self.f2_ready = self.f2.poll(context).is_ready();
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
        f1,
        f1_ready: false,
        f2,
        f2_ready: false,
    }
}

fn main() {
    let mut main_future = join(work("foo", 1.5), work("bar", 2.0));
    let mut context = Context {};
    while main_future.poll(&mut context).is_pending() {
        let next = NEXT_WAKE_TIME.get().expect("someone must want a wakeup");
        let duration = next.saturating_duration_since(Instant::now());
        NEXT_WAKE_TIME.set(None);
        std::thread::sleep(duration);
    }
}
