use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        println!("1) Convert Celsius to Fahrenheit");
        println!("2) Convert Fahrenheit to Celsius");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read your input");

        // Hack to avoid accepting the wrong option (neither 1 or 2)
        let option: u8 = match option.trim().parse() {
            Ok(1) => 1,
            Ok(2) => 2,
            Ok(_) => continue,
            Err(_) => continue,
        };

        println!("Please input the source value");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your input");

        let source: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
           let farenheit : f64 = (source * 1.8) + 32.0;
           println!("{} Celsius is {} in Fahrenheit", source, farenheit);
        } else {
           let celsius : f64 = (source - 32.0) * 1.8;
           println!("{} Fahrenheit is {} Celsius", source, celsius);
        }
    }
}
