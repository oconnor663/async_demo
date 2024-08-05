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
    rayon::join(foo, bar);
}
