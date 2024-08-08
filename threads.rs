use std::io::Write;
use std::time::Duration;

fn work() {
    std::thread::sleep(Duration::from_secs(1));
    print!(".");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let thread1 = std::thread::spawn(work);
    let thread2 = std::thread::spawn(work);
    let thread3 = std::thread::spawn(work);
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    println!();
}
