use std::time::Duration;

fn work(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

fn main() {
    work(1);
    work(2);
    work(3);
}
