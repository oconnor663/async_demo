use std::io::Write;
use std::time::Duration;

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    print!(".");
    std::io::stdout().flush().unwrap();
}

fn main() {
    work();
    work();
    work();
    println!();
}
