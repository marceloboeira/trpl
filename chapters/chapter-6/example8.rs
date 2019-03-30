#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn to_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
   println!("{} cents", to_cents(Coin::Penny{}));
   println!("{} cents", to_cents(Coin::Nickel{}));
   println!("{} cents", to_cents(Coin::Dime{}));
   println!("{} cents", to_cents(Coin::Quarter(UsState::Alabama)));
}
