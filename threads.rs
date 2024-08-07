use std::time::Duration;

fn work(name: &str, seconds: f32) {
    let duration = Duration::from_secs_f32(seconds);
    println!("{name}: start");
    std::thread::sleep(duration / 2);
    println!("{name}: middle");
    std::thread::sleep(duration / 2);
    println!("{name}: end");
}

fn main() {
    let t1 = std::thread::spawn(|| work("foo", 1.5));
    let t2 = std::thread::spawn(|| work("bar", 2.0));
    t1.join().unwrap();
    t2.join().unwrap();
}
