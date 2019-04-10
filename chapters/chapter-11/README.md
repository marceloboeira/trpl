# Chapter 11

## How to Write tests?

The bodies of test functions typically perform these three actions:

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

Let's look at the features Rust provides specifically for writing tests that take these actions, which include the test attribute, a few macros, and the should_panic attribute.


### The Anatomy of a Test Function


 To change a function into a test function, add `#[test]` on the line before fn.

 When you run your tests with the cargo test command, Rust builds a test runner binary that runs the functions annotated with the test attribute and reports on whether each test function passes or fails.

See `adder/src/lib.rs`.

Tests fail if the running block fail, or if assertions endup being false.

### Checking Results with the `assert!` Macro

The `assert!` macro expects a value to be `true`, if it's false it `panic!`s.


### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

The `assert_eq!` and `assert_ne!` macros test 2 values for Equality.


### Adding Custom Failure Messages

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

### Checking for Panics with `should_panic`

```rust
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
```

### Using `Result<T, E>` in tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Controlling How Tests Are Run

### Running Tests in Parallel or Consecutively

When you run multiple tests, by default they run in parallel using threads.

`cargo test -- --test-threads=1` sets the amount of threads the test suite can use, let's say you want to use only 1, or maybe a specific number...

### Showing Function Output

By default, if a test passes, Rust's test library captures anything printed to standard output. To show those values, you can set:

`cargo test -- --nocapture`

### Running subset

Running a Subset of Tests by Name

#### Single Tests

`cargo test $name`

#### Filtering to Run Multiple Tests

`cargo test $prefix`

#### Ignoring Some Tests Unless Specifically Requested

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

The tests flagged with `ignore` will only run if you ask:

`cargo test -- --ignored`

## Test Organization

### Unit Tests

Test each component as an isolated part of your project.

#### The Test Module

`#[cfg(test)]` makes sure your tests are only compiled when you run `cargo test` and not `cargo run` or `cargo build`.

#### Testing Private Functions

You should not, but rust allows you to do it, if you really want it....

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

#### Integration Tests

You can create a `tests` folder, near the `src` folder, so that the integration tests can be created underneath that directory.

`cargo test --test integration_test`

If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.
