# Chapter 12

## An I/O Project: Building a Command Line Program

## Accepting Command Line Arguments

The first task is to make minigrep accept its two command line arguments: the filename and a string to search for. That is, we want to be able to run our program with cargo run, a string to search for, and a path to a file to search in, like so:

`cargo run searchstring example-filename.txt`

### Reading the Argument Values

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```
First, we bring the std::env module into scope with a use statement so we can use its args function.

#### The args Function and Invalid Unicode

Note that std::env::args will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead. That function returns an iterator that produces OsString values instead of String values. We’ve chosen to use std::env::args here for simplicity, because OsString values differ per platform and are more complex to work with than String values.

```rust
$ cargo run
--snip--
["target/debug/minigrep"]

$ cargo run needle haystack
--snip--
["target/debug/minigrep", "needle", "haystack"]
```

### Saving the Argument Values in Variables

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

```rust
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```

## Reading a File

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

In main, we’ve added a new statement: fs::read_to_string will take the filename, open that file, and then return Result<String> with its contents.

```
$ cargo run foo ./fixtures/poem.txt
  Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
      Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

### Refactoring to Improve Modularity and Error Handling

First, our main function now performs two tasks: it parses arguments and reads files.

For such a small function, this isn’t a major problem. However, if we continue to grow our program inside main, the number of separate tasks the main function handles will increase. As a function gains responsibilities, it becomes more difficult to reason about, harder to test, and harder to change without breaking one of its parts. It’s best to separate functionality so each function is responsible for one task.

This issue also ties into the second problem: although query and filename are configuration variables to our program, variables like contents are used to perform the program’s logic. The longer main becomes, the more variables we’ll need to bring into scope; the more variables we have in scope, the harder it will be to keep track of the purpose of each. It’s best to group the configuration variables into one structure to make their purpose clear.

The third problem is that we’ve used expect to print an error message when reading the file fails, but the error message just prints something went wrong. Reading a file can fail in a number of ways: for example, the file could be missing, or we might not have permission to open it. Right now, regardless of the situation, we’d print the something went wrong error message, which wouldn’t give the user any information!

Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. It would be best if all the error-handling code were in one place so future maintainers had only one place to consult in the code if the error-handling logic needed to change. Having all the error-handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

### Separation of Concerns for Binary Projects

* Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
* As long as your command line parsing logic is small, it can remain in main.rs.
* When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
* The responsibilities that remain in the main function after this process should be limited to the following:
  * Calling the command line parsing logic with the argument values
  * Setting up any other configuration
  * Calling a run function in lib.rs
  * Handling the error if run returns an error

### Extracting the Argument Parser

```rust
fn main() {
  let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```
