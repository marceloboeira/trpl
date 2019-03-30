struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle::square(2);

    println!("Rect 1 w: {}, h: {}", rect1.width, rect1.height);
}
