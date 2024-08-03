use std::thread::sleep;
use std::time::Duration;

fn foo() {
    println!("foo start");
    sleep(Duration::from_secs_f64(2.5));
    println!("foo end");
}

fn bar() {
    println!("bar start");
    sleep(Duration::from_secs_f64(2.0));
    println!("bar end");
}

fn join(f1: fn(), f2: fn()) {
    let thread1 = std::thread::spawn(f1);
    let thread2 = std::thread::spawn(f2);
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn main() {
    join(foo, bar);
}
