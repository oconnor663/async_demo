use std::time::Duration;

fn job(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

fn main() {
    let thread1 = std::thread::spawn(|| job(1));
    let thread2 = std::thread::spawn(|| job(2));
    let thread3 = std::thread::spawn(|| job(3));
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}
