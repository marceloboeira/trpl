use std::collections::HashMap;

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
            Message::ChangeColor(a, b, c) => {
                println!("ChangeColor with a: {}, b: {}, c: {}", a, b, c)
            }
        }
    }
}

fn main() {
    let mut index = HashMap::new();

    index.insert("first", Message::Move { x: 10, y: 20 });
    index.insert("second", Message::Write(String::from("marcelo")));
    index.insert("third", Message::ChangeColor(10, 20, 30));
    index.insert("fourth", Message::Quit);

    match index.get("first") {
        Some(m) => m.call(),
        None => println!("Nothing"),
    }
    index.insert("first", Message::Write(String::from("marcelo"))); //overrides first

    scores.entry("second").or_insert(Message::Write(String::from("marcelo"))); // writes if non-existent
    scores.entry("fifth").or_insert(Message::Write(String::from("marcelo"))); // write since non-existent

    println!("All messages:");
    for (k, v) in &index {
        v.call()
    }
}
