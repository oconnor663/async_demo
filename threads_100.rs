use std::time::Duration;

fn job(n: u64) {
    std::thread::sleep(Duration::from_secs(1));
    println!("{n}");
}

fn main() {
    let mut threads = Vec::new();
    for n in 1..=100 {
        threads.push(std::thread::spawn(move || job(n)));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
