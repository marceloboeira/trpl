use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..100 {
            println!("T1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(|| {
        for i in 1..100 {
            println!("T2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait 1 second, for the other threads to finish
    thread::sleep(Duration::from_millis(1000));
}
