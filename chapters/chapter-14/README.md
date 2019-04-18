# Chapter 14

## More About Cargo and Crates.io

* Customize your build through release profiles
* Publish libraries on crates.io
* Organize large projects with workspaces
* Install binaries from crates.io
* Extend Cargo using custom commands

## Customizing Builds with Release Profiles

Cargo has two main profiles: the `dev` profile Cargo uses when you run cargo build and the release profile Cargo uses when you run `cargo build --release`.

The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.


```
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Publishing a Crate to Crates.io

### Making Useful Documentation Comments

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

```shell
cargo doc --open
```

### Commonly Used Sections

* Panics: The scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
* Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
* Safety: If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

### Documentation Comments as Tests


By default, cargo test runs the code under the comments with assert_eq as a way of checking if the examples do work!
```
cargo test
```

```
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Commenting Contained Items

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

Adds general documentation rather than method specific examples and such

### Exporting a Convenient Public API with pub use

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

## Setting Up a Crates.io Account

```shell
cargo login abcdefghijklmnopqrstuvwxyz012345
```

### Adding Metadata to a New Crate

```toml
[package]
name = "example"
version = "0.1.0"
authors = ["Marcelo Boeira <me@marceloboeira.com>"]
edition = "2018"
license = "MIT"

[dependencies]
```

### Publishing to Crates.io

```shell
cargo publish
```

### Removing Versions from Crates.io with cargo yank

```shell
cargo yank --vers 1.0.1
```

```shell
cargo yank --vers 1.0.1 --undo
```

## Cargo Workspaces

In Chapter 12, we built a package that included a binary crate and a library crate. As your project develops, you might find that the library crate continues to get bigger and you want to split up your package further into multiple library crates. In this situation, Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.


### Creating a Workspace

See `add/`.

```toml
[workspace]

members = [
    "adder",
]
```

```
cargo new adder
```

### Creating the Second Crate in the Workspace

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

```
cargo new add-one --lib
```

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

```shell
cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

### Depending on an External Crate in a Workspace

```toml
[dependencies]

rand = "0.3.14"
```

### Adding a Test to a Workspace

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

```
$ cargo test
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/deps/add_one-f0253159197f7841

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-f88af9d2cc175a5e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
### Installing Binaries from Crates.io with `cargo install`

The cargo install command allows you to install and use binary crates locally. This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on crates.io.

```
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 --snip--
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg
```

###Extending Cargo with Custom Commands

Cargo is designed so you can extend it with new subcommands without having to modify Cargo. If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running cargo something. Custom commands like this are also listed when you run cargo --list. Being able to use cargo install to install extensions and then run them just like the built-in Cargo tools is a super convenient benefit of Cargo’s design!

Sharing code with Cargo and crates.io is part of what makes the Rust ecosystem useful for many different tasks. Rust’s standard library is small and stable, but crates are easy to share, use, and improve on a timeline different from that of the language. Don’t be shy about sharing code that’s useful to you on crates.io; it’s likely that it will be useful to someone else as well! 
