fn largest<T>(list: &[T]) -> T {
    let mut l = list[0];

    for &item in list.iter() {
        if item > l {
            l = item;
        }
    }

    l
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
