fn sum_optionals(a: Option<i8>, b: Option<i8>) -> i8 {
    match (a, b) {
        (None, None) => 0,
        (Some(x), Some(y)) => x + y,
        (Some(x), None) => x,
        (None, Some(x)) => x,
    }
}

fn main() {
    println!("{}", sum_optionals(None, None));
    println!("{}", sum_optionals(Some(10), None));
    println!("{}", sum_optionals(None, Some(20)));
    println!("{}", sum_optionals(Some(20), Some(30)));
}
