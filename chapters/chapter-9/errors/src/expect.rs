use std::fs::File;

fn main() {
    let _f = File::open("hello_2.txt").expect("Failed to open hello.txt");
}
