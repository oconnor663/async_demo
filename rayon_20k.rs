use std::time::Duration;

fn job(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

fn main() {
    rayon::scope(|scope| {
        for n in 1..=20_000 {
            scope.spawn(move |_| job(n));
        }
    });
}
