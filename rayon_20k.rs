use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Duration;

static X: AtomicU64 = AtomicU64::new(0);

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    X.fetch_add(1, Relaxed);
}

fn main() {
    rayon::scope(|scope| {
        for _ in 0..20_000 {
            scope.spawn(|_| work());
        }
    });
    println!("X: {}", X.load(Relaxed));
}
