fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //println!("s1 {}", s1); //doesnt work anymore, because it was moved
    println!("s2 {}", s2); // works
    println!("s3 {}", s3); // works
}
