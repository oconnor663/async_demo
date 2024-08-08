use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::{Duration, Instant};

static X: AtomicU64 = AtomicU64::new(0);

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    X.fetch_add(1, Relaxed);
}

fn lots_of_work() {
    let thread1 = std::thread::spawn(work);
    let thread2 = std::thread::spawn(work);
    let thread3 = std::thread::spawn(work);
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}

fn main() {
    let start = Instant::now();
    lots_of_work();
    println!("X is {:?}", X);
    let seconds = (Instant::now() - start).as_secs_f32();
    println!("{:.3} seconds", seconds);
}
