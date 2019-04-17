fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let mut v1_iter2 = v1.iter();

    assert_eq!(v1_iter2.next(), Some(&1));
    assert_eq!(v1_iter2.next(), Some(&2));
    assert_eq!(v1_iter2.next(), Some(&3));
    assert_eq!(v1_iter2.next(), None);
}
