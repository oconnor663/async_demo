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
    work("foo", 1.5);
    work("bar", 2.0);
}
