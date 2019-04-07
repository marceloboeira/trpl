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
