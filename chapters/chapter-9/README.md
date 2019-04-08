# Chapter 9

# Error Recovery

There are recoverable and unrecoverable errors.

## Unrecoverable Errors with `panic!`

`panic!` macro wipes the stack and exits with the given error message.

If we want to have a smaller binary, we can ignore that behavior and let the OS do this job by declaring that our `panic!` behavior is to abort:

```toml
[profile.release]
panic = "abort"
```

(`awsudo` got a 2MB reduction on binary...)

### Crash and Burn

See `errors/src/main1.rs`;

Basic example of `panic!` with short explanation how the macro expands to exit the program.

### Index


```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Here, we’re attempting to access the 100th element of our vector (which is at index 99 because indexing starts at zero), but it has only 3 elements. In this situation, Rust will panic. Using [] is supposed to return an element, but if you pass an invalid index, there’s no element that Rust could return here that would be correct.

### Buffer Overread

Other languages, like C, will attempt to give you exactly what you asked for in this situation, even though it isn’t what you want: you’ll get whatever is at the location in memory that would correspond to that element in the vector, even though the memory doesn’t belong to the vector. This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the array.

Rust stops the execution to prevent such security issues:

```rust
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/slice/mod.rs:2455:10
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

The error point to `mod.rs:2455:10` which is where the `panic!` is called from.

### Backtrace

You can set the RUST_BACKTRACE environment variable to get a backtrace of exactly what happened to cause the error. A backtrace is a list of all the functions that have been called to get to this point

 In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enabled by default when using cargo build or cargo run without the --release flag.

```
 RUST_BACKTRACE=1 cargo run --bin main2
```

## Recoverable Errors

For errors that are somewhere expected, anything with side-effects, opening files, network requests... we have `Result`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

* `T` is the expected type, wrapped because it could be missing
* `E` is the error type.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => println!("{:?}", file),
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

### Matching on Different Errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
```

We can specify the reason for `File::open` to fail, for instance: `ErrorKind::NotFound`. There could be other, such as permission, corruption ...

### Shortcuts for Panic on Error: `unwrap` and `expect`

Unwrap, forces the value of `T` under `Result<T, E>` to be extracted, or `panic!` if missing.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

Expect does pretty much the same, yet it fails with a given error string.
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

### Propagating Errors

When creating functions we can avoid handling the errors and just raise them to the calling block.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
#### A Shortcut for Propagating Errors: the `?` Operator

The above thing can easily be replaced with:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

The main difference is that ? makes the error pass through the From trait, which packes it to the return error of the running function.

The `?` operator eliminates a lot of boilerplate, but the type needs to implement the `From` trait.

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### The `?` Operator Can Only Be Used in Functions That Return `Result`

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

## To `panic!` or Not to `panic!`

When to panic:

* The operation is essential for the running code
* On Prototypes and tests
* When you have more info than the compiler `let home: IpAddr = "127.0.0.1".parse().unwrap();` (you know that it won't fail).
* When an unexpected failure happen

When not to panic:

* When you want to give freedom to the caller of a function on how to react to failure(s), e.g.: library code, functions, abstractions...
* When expected failures happen (things that could go wrong frequently, network requests, file loading, IO related things...)


#### Using the type system for handling errors

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}
```

becomes:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
