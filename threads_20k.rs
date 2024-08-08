use std::io::Write;
use std::time::Duration;

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    print!(".");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let mut threads = Vec::new();
    for _ in 0..20_000 {
        threads.push(std::thread::spawn(work));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!();
}
