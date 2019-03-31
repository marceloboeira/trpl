fn main() {
    let len = String::from("Hola").len();
    let len2 = String::from("Здравствуйте").len();
   
    println!("{}", len);
    println!("{}", len2);
    println!("{:?}", String::from("Здравствуйте").bytes());
    println!("{:?}", String::from("नमस्ते").bytes());
}
