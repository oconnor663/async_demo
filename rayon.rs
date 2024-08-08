use std::io::Write;
use std::time::Duration;

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    print!(".");
    std::io::stdout().flush().unwrap();
}

fn main() {
    rayon::scope(|scope| {
        for _ in 0..20_000 {
            scope.spawn(|_| work());
        }
    });
    println!();
}
