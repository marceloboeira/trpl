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
