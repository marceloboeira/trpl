fn main() {
    let v1 : Vec<i32> = vec![1, 2, 3];
    let v2 :  Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("Sum: {:?}", v2);
}
