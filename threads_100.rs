use std::time::Duration;

fn work(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

fn main() {
    let mut threads = Vec::new();
    for n in 1..=100 {
        threads.push(std::thread::spawn(move || work(n)));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
