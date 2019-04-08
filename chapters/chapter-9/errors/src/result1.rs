use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => println!("{:?}", file),
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
