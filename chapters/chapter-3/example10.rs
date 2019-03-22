fn max_ten(x: i32) -> i32 {
    if x > 10 {
        return 10
    }

    x
}

fn main() {
    let x = max_ten(12);

    println!("The value of x is: {}", x);
}
