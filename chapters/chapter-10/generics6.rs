struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn y(&self) -> i32 {
        self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p: Point<i32> = Point { x: 5, y: 10 };
    let f: Point<f32> = Point { x: 5.0, y: 10.0 };

    println!("p.y = {}", p.y());
    println!("f.distance = {}", f.distance_from_origin());
}
