use std::time::Duration;

fn foo() {
    println!("foo start");
    std::thread::sleep(Duration::from_secs_f64(2.5));
    println!("foo end");
}

fn bar() {
    println!("bar start");
    std::thread::sleep(Duration::from_secs_f64(2.0));
    println!("bar end");
}

fn main() {
    let t1 = std::thread::spawn(foo);
    let t2 = std::thread::spawn(bar);
    t1.join().unwrap();
    t2.join().unwrap();
}
