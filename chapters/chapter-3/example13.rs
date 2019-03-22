fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        // "six"
        6
    };

    println!("The value of number is: {}", number);
}
