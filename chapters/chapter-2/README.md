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
