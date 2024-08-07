use std::time::Duration;

fn sleep(seconds: f64) {
    let duration = Duration::from_secs_f64(seconds);
    std::thread::sleep(duration);
}

fn foo() {
    println!("foo start");
    sleep(0.5);
    println!("foo middle");
    sleep(1.0);
    println!("foo end");
}

fn bar() {
    println!("bar start");
    sleep(1.0);
    println!("bar middle");
    sleep(1.0);
    println!("bar end");
}

fn main() {
    foo();
    bar();
}
