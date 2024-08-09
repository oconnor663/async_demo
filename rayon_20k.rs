use std::io::Write;
use std::time::Duration;

fn work(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    print!("{n} ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    rayon::scope(|scope| {
        for n in 1..=20_000 {
            scope.spawn(move |_| work(n));
        }
    });
    println!();
}
