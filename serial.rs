use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::{Duration, Instant};

static X: AtomicU64 = AtomicU64::new(0);

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    X.fetch_add(1, Relaxed);
}

fn lots_of_work() {
    work();
    work();
    work();
}

fn main() {
    let start = Instant::now();
    lots_of_work();
    println!("X is {X:?}");
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{seconds:.3} seconds");
}