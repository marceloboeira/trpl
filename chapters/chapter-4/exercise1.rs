// Ugly way of doing it...

fn main() {
    let word1 = first_word(&String::from("marcelo boeira"));
    let word2 = first_word(&String::from("marceloboeira"));
    let word3 = first_word(&String::from(" foo"));

    println!("{}", word1);
    println!("{}", word2);
    println!("{}", word3);
}

fn first_word(s: &String) -> String {
    let mut word = String::from("");

    for c in s.chars() {
        if c == ' ' {
            break
        }
        else {
            word.push(c)
        }
    }

    word
}
