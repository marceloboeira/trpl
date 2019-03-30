struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move with x: {} y: {}", x, y),
            Message::Write(m) => println!("Write with {}", m),
            Message::ChangeColor(a, b, c) => println!("ChangeColor with a: {}, b: {}, c: {}", a, b, c),
        }
    }
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move{x: 10, y: 20},
        Message::Write(String::from("marcelo")),
        Message::ChangeColor(10, 20, 30),
    ];

    for m in messages.iter() {
        m.call()
    }
}
