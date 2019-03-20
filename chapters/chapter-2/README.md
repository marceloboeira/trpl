# Chapter 2

## Guessing Game
> Only highlighting interesting/new things.

### IO/Prelude
First, import the IO lib, which comes from the standard library (also known as std):
```rust
use std::io;
```

IO is not on the prelude. More info about the Prelude: https://doc.rust-lang.org/std/prelude/index.html

### Let Statements
> Let allows to create variables

In Rust, variables are immutable by default, so there is a need to explicitly request a mutable variable.
```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```

PS: `//` is a comment.

```rust
let mut guess = String::new();
```

The `::` syntax in the `::new` line indicates that new is an `associated function` of the `String` type.

### Using IO

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

Like variables, references are immutable by default.  Hence, you need to write `&mut` guess rather than `&guess` to make it mutable.

### Result Type

`read_line` returns an `io::Result`, which is an [enum](https://doc.rust-lang.org/book/ch06-00-enums.html) with the variants `Ok`, and `Err`.

As the [method signature](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect) implies, the `expect` method Unwraps a result, yielding the content of Ok.

If you don't call expect, you'll get a warning:

```rust
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

Rust warns that you haven’t used the Result value returned from read_line, indicating that the program hasn’t handled a possible error.

### Printing variables

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

Just an example on how to print stuff.

### Adding dependencies

Update Cargo.toml.

```toml
[dependencies]

rand = "0.3.14"
```

Crates are rust packages, and [crates.io](https://crates.io) is where people publish them.

### Making it random

```rust
use rand::Rng;
```

To import random functionality.

```rust
let secret_number = rand::thread_rng().gen_range(1, 101);
```

* The rand::thread_rng function will give us the particular random number generator that we're going to use: one that is local to the current thread of execution and seeded by the operating system.
*  The gen_range method takes two numbers as arguments and generates a random number between them. It’s inclusive on the lower bound but exclusive on the upper bound, so we need to specify 1 and 101 to request a number between 1 and 100.

### Cargo docs

```shell
cargo doc --open
```

### Comparing values / Parsing strings

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");

println!("You guessed: {}", guess);

match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too small!"),
  Ordering::Greater => println!("Too big!"),
  Ordering::Equal => println!("You win!"),
}
```

* More info on the parse method: https://doc.rust-lang.org/std/primitive.str.html#method.parse
* Rust allows shaddowing the guess (String) to a guess (u32).
* Expect to unwrap the value
* Like `Result`, `Ordering` is another enum, but the variants for `Ordering` are `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.
* Pattern matching - https://doc.rust-lang.org/1.4.0/book/patterns.html

### Loop Logic

* `loop` - initializes a loop flow
* `break` - breaks out of the loop
* `continue` - jumps to the next loop run
