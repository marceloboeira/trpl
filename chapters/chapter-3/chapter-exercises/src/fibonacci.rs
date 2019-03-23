use std::io;

fn main() {
    println!("Fibonacci nth number");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read your input");

    let result : i32 = match n.trim().parse() {
        Ok(num) => fibonacci(num),
        Err(_) => panic!("Foo"),
    };

    println!("The {}th fibonacci number is {}", n.trim(), result);
}

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 2,
        n => fibonacci(n-1) + fibonacci(n-2)
    }
}
