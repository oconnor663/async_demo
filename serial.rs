use std::io::Write;
use std::time::Duration;

fn work(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    print!("{n} ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    work(1);
    work(2);
    work(3);
    println!();
}
