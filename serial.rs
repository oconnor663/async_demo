use std::time::Duration;

fn job(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

fn main() {
    job(1);
    job(2);
    job(3);
}
