use std::io::Write;
use std::time::Duration;

fn work(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    print!("{n} ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let mut threads = Vec::new();
    for n in 1..=20_000 {
        threads.push(std::thread::spawn(move || work(n)));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!();
}
