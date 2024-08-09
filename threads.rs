use std::io::Write;
use std::time::Duration;

fn work(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    print!("{n} ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let thread1 = std::thread::spawn(|| work(1));
    let thread2 = std::thread::spawn(|| work(2));
    let thread3 = std::thread::spawn(|| work(3));
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    println!();
}
