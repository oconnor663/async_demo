use std::time::Duration;

fn job(n: u64) {
    println!("start {n}");
    std::thread::sleep(Duration::from_secs(1));
    println!("end {n}");
}

fn main() {
    rayon::scope(|scope| {
        for n in 1..=1_000 {
            scope.spawn(move |_| job(n));
        }
    });
}
