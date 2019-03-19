# Chapter 1

## Getting Started

### Installing rust

It's better to use rustup, for macOS:

```shell
brew install rustup
```

And to initialize rustup:

```shell
rustup-init
```

### Hello World

See `hello_world/main.rs`.

```shell
rustc hello_world/main.rs
```

### Syntax details

`println!` calls a Rust macro. If it called a function instead, it would be entered as `println` (without the `!`).

### rustfmt

A tool to make sure rust is formated properly.


### Hello Cargo

Cargo - build system and package manager.

Create a new project:
```shell
cargo new hello_cargo
```

Build:
```shell
cargo build

./target/debug/hello_cargo
```

Or you can also:

```shell
cargo run
```
