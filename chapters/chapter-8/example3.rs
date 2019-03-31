enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(7),
        SpreadsheetCell::Float(21.04),
        SpreadsheetCell::Text(String::from("yellow")),
        SpreadsheetCell::Float(29.01),
    ];

    for r in row {
        match r {
            SpreadsheetCell::Int(i) => println!("This is an Int cell, with value: {}", i),
            SpreadsheetCell::Float(f) => println!("This is a Float cell, with value: {}", f),
            SpreadsheetCell::Text(t) => println!("This is an Text cell, with value: {}", t),
        }
    }
}
