# Chapter 6

# Enums
> Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to algebraic data types in functional languages, such as OCaml, and Haskell.

## Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

> V4 and V6 are known as the variants

`IpAddrKind` is now a custom data type that we can use elsewhere in our code.

See `example1.rs`.

## Enum Values

We can write an IP Address like this:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```
See `example2.rs`.

Or, embedding the type into an single enum with data...

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

See `example3.rs`.

We can also use different types of data, for different purposes. Let's say you want to limit ipv4 from 0 to 255, as the original protocol, you can use `u8`.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

See `example4.rs`.

if we need to be even more specific, we can also:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

References: [std::net::IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html).

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enum has four variants with different types:

* `Quit` - has no data associated with it at all.
* `Move` - includes an anonymous struct inside it.
* `Write` - includes a single `String`.
* `ChangeColor` - includes three `i32` values.

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

See `example5.rs`.

## The `Option` Enum and Its Advantages Over `Null` Values

Usual Optional introduction...

```rust
enum Option<T> {
    Some(T),
    None,
}
```

* The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
* In addition, so are its variants: you can use `Some` and `None` directly without the `Option::` prefix.
* The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of type `Option<T>`.
* Generics will be covered on a later chapter.

Option is strongly typed and it errors on compile time when things don't match...

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

We could write something like:

```rust
fn sum_optionals(a: Option<i8>, b: Option<i8>) -> i8 {
    match (a, b) { // we use a tuple so we can pattern match over both params...
        (None, None) => 0, // default value...
        (Some(x), Some(y)) => x + y, // happy path
        (Some(x), None) => x, // Left only
        (None, Some(x)) => x, // Right only
    }
}

fn main() {
    println!("{}", sum_optionals(None, None)); // 0
    println!("{}", sum_optionals(Some(10), None)); // 10
    println!("{}", sum_optionals(None, Some(20))); // 20
    println!("{}", sum_optionals(Some(20), Some(30))); // 50
}
```

## The `match` Control Flow Operator

Match is really powerful, it allow us to control the flow based on types/enums of our program.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn to_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

When match runs, it runs our expressions agains all the branches...

There could be more into the branch itself...

```rust
 match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
```

See `example7.rs`.

## Patterns that Bind to Values

```rust
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
```

See `example8.rs`.

## Matching with `Option<T>`

Example, from Option to Option...

if something is there, `Some(x)`, we sum, otherwise, we return `None`.
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

See `example6.rs`.

## Matches Are Exhaustive

One of the magics of `pattern matching` is that the compiler knows of your options and it forces your to make impossible states impossible...

if you try something like:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

you haven't matched all the patterns, you'll see something like:

```
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

## The `_` placeholder

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

Since patterns are exhaustive, for things un countless possibilities, you might take advantage of `_` for default behavior.
Yet, try to avoid as much as possible.

In a situation in which we care about only one of the cases. For this situation, Rust provides if let.

## Concise Control Flow with `if let`

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

If you only want to use the value to compare and print or not, you can use:

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

You can also have else, for default values:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```


```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
