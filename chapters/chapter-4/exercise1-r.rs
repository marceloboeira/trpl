// Rust way of doing it...

fn main() {
    let m1 = String::from("marceloboeira");
    let m2 = String::from("marcelo boeira");
    let m3 = String::from(" boeira");

    let w1 = first_word(&m1);
    let w2 = first_word(&m2);
    let w3 = first_word(&m3);

    println!("{}", w1);
    println!("{}", w2);
    println!("{}", w3);
}

fn first_word(s: &String) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i]
        }
    }

    &s[..]
}
