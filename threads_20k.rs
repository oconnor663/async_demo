use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Duration;

static X: AtomicU64 = AtomicU64::new(0);

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    X.fetch_add(1, Relaxed);
}

fn main() {
    let mut threads = Vec::new();
    for _ in 0..20_000 {
        threads.push(std::thread::spawn(work));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!("X: {}", X.load(Relaxed));
}
