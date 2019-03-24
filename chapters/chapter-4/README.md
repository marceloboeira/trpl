# Chapter 4

## Understanding Ownership

### What is Ownership?

* Rust's central feature is ownership. Although the feature is straightforward to explain, it has deep implications for the rest of the language.
* A way of managing memory that doesn't envolve custom allocation nor garbage collection from the runtime.


### Rules

* Each value in Rust has a variable that's called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Variable Scope

```rust
{                      // s is not valid here, it's not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}
// this scope is now over, and s is no longer valid
```

* When s comes into scope, it is valid.
* It remains valid until it goes out of scope.

## String

* string literals, not `String`, are:
  * stored on the stack
  * handled in compile time (can't change/immutable)
* `String` is
  * allocated on the heap
  * (optionally) mutable
  * can change in size / have size defined during runtime

You can create `String` from literals with `String::from`:

e.g.:

```rust
let s = String::from("hello");
```

* The double colon `::` is an operator that allows us to namespace this particular from function under the `String`.

And you can also, mutate them, for concatenation ...

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

Why can `String` be mutated but `literals` cannot? The difference is how these two types deal with memory.

## Memory and Allocation

Literals are baked into the binary during compiling

For unkown sized attributions, such as `String` we need to:

* request memory from the operating system at runtime.
* a way of returning this memory to the operating system when we're done with our `String`.

The first part, is faily easy, whereas the second is a bit more of a hassle.

In languages with a garbage collector (GC), the GC keeps track and cleans up memory that isn't being used anymore, and we don't need to think about it.

The downside, is that the GC has to run frequently, the GC can be a blocking procedure and for memory intensive programs the GC can take a long time to run.

Rust approach, as mentioned before, doens't involve a GC.

Without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it.

Doing this correctly has historically been a difficult programming problem:
* If we forget, we'll waste memory.
* If we do it too early, we'll have an invalid variable.
* If we do it twice, that's a bug too.

We need to pair exactly one allocate with exactly one free.

Rust takes a different path: the memory is **automatically** returned once the variable that owns it goes out of scope.

e.g.:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward || malloc(...

    // do stuff with s
}                                  // this scope is now over, and s is no || free(...
                                   // longer valid
```

* When the scope starts, we declare the `String`, from the literal (the String gets allocated on the heap)
* We use the string, for our purposes...
* We finish the block / scope, soon, the String is not reacheable anymore, right at this time the compiler adds notation so that we drop / free the memory, returning it to the system.
  * Important to notice that the "drop" or de-allocation is done by the struct itself...
    * Explore how drop works in Rust: https://doc.rust-lang.org/std/mem/fn.drop.html
    * Trait for dropping" https://doc.rust-lang.org/std/ops/trait.Drop.html

### Move
> Multiple variables can interact with the same data in different ways in Rust

```rust
let x = 5;
let y = x;
```

* bind the value 5 to x;
* then make a copy of the value in x and bind it to y;
* we now have two variables, x and y, and both equal 5;
* integers are simple values with a **known, fixed size**, and these two 5 values are **pushed onto the stack**;

Now, with String...

```rust
let s1 = String::from("hello");
let s2 = s1;
```

We might think it's the same, but it's not.

A String is made up of three parts:
* a pointer to the memory that holds the contents of the string
* a length - how much memory, in bytes, the contents of the String is currently using
* a capacity - total amount of memory, in bytes, that the String has received from the operating system

This group of data is stored on the stack. Whereas, the data itself is stored on the heap, since its size is unknown at compile time and variable throughout the program lifetime.

```
STACK            | HEAP
                 |
NAME | VALUE     |
ptr  | ----------|---->	INDEX | VALUE
len  | 5         |      0     | 'h'
cap  | 5         |      1     | 'e'
                 |      2     | 'l'
                 |      3     | 'l'
                 |      4     | 'o'
```

When we assign s1 to s2, the String data is copied
* we copy the pointer
* the length
* the capacity
* We do not copy the data on the heap that the pointer refers to.

```
STACK            | HEAP
                 |
     S1          |
NAME | VALUE     |
ptr  | ----------|--|->	INDEX | VALUE
len  | 5         |  |   0     | 'h'
cap  | 5         |  |   1     | 'e'
                 |  |   2     | 'l'
                 |  |   3     | 'l'
                 |  |   4     | 'o'
     S2          |  |
NAME | VALUE     |  |
ptr  | ----------|--|
len  | 5         |
cap  | 5         |
```

If rust were to copy, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.

When a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.
Since both data pointers pointing to the same location, there is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.

This is known as a **double free error**. Freeing memory twice can lead to **memory corruption**, which can potentially lead to security vulnerabilities.

To avoid such an issue, rust considers `s1` to not be valid anymore,

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

3 important terms:

* deep copy - copying or cloning the full value on the heap and stack
* shallow copy - copying only the stack and pointing to the same heap
* move - copying only the stack, as the shallow copy, but also invalidates the original stack value.

Rust will never automatically create **deep** copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

### Clone

Same considerations as move, but instead of invalidating the old stack and pointing to the new heap, it copies the full thing on stack and heap, having a deep-copy or as the name implies, a clone. The use is reasonable when you need to use both instances after the fact...

e.g.:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.

### ~Stack~ Copy

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

The code seems to be contradictory, since we don't clone but we can still use x and y, but the thing is that they are on the stack, because their size is known at compile time, which is quick enough. i.e.: copying a pointer or an integer should have the same "cost".

For that, Rust has the `Copy` trait, if the type implements that trait Rust will let you use the value even after a copy. However, if the type has a `Drop` trait instance, we can't anotate it with `Copy`.


As a general rule, any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`. Here are some of the types that are `Copy`:

* All the integer types, such as `u32`.
* The `Boolean` type, bool, with values true and false.
* All the floating point types, such as `f64`.
* The character type, `char`.
* `Tuples`, if they only contain types that are also Copy. For example, `(i32, i32)` is `Copy`, but `(i32, String)` is not.

## Ownership and Functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

* Passing a variable to a function will move or copy, just as assignment does.

### Return Values and Scope

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

You can return multiple values from a function, in order to get the value back, if necessary.

e.g.:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Even though, it's a bit tedious and burocratical, so, Luckily for us, Rust has a feature for this concept, called references.

## Reference and Borrowing

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

We now give `&s1` to the function and the function signature is expecting `&String`, `&` means `Referrence`.

References allow a scope to refer to some value without taking ownership of it.

```
     s				|      s1      |   HEAP
              |              |
NAME | VALUE  | NAME | VALUE | INDEX | VALUE
 ptr | --------> ptr | ----------> 0 | 'h'
NAME | VALUE  |  len | 5     |     1 | 'e'
              |  cap | 5     |     2 | 'l'
              |              |     3 | 'l'
              |              |     4 | '0'
```

The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

* When we use a reference the compiler won't drop the value after "borrowing it"
* There is no need to return the value, since the function never owned it, just a reference
* You can't modify it, because it's immutable
* Rust calls having references as function parameters **borrowing**

```rust

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, nothing happens.
```

We cant't mutate it...

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.

### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

* Update the `let` to be `mut`
* Update the reference given, to be a mutable reference of s
* Update the function type signature to expect a reference of a mutable String

Everything works, as expected. However, there is a limitation: you can have only one mutable reference to a particular piece of data in a particular scope.

This code will fail:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

**Prevents data racing** - A data race is similar to a race condition and happens when these three behaviors occur:
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There's no mechanism being used to synchronize access to the data.

The limitation is per scope, so with multiple contexts/scopes, should be reasonable to allow that:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Important to notice that, this doens't apply to immutable references:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

`r1` and `r2` are okay, nothing breaks if 2 things are reading from the same immutable reference, whereas, `r3` creates a problem, since you have a 3rd thing being able to update that reference while other are reading from it (on the same scope).

### Dangling References

A dangling reference is something quite common on languages with pointers. Basically, it's the result of a pointer that still exists, even though it's referred memory was already freed. In rust, the compiler guaratees that this doesn't exist anymore.

e.g.:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

* the let is trying to get the return of `dangle()`
* dangle creates a `String`, and tries to return its reference, yet, since the `String` was created on its scope, should it leave outside of it? we would have then returned a reference to a String that had to be deallocated by the end of the scope.

The solution would be to give ownership, by returning the full value itself.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

Ownership is moved out, and nothing is deallocated.

## The Slice Type

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

* we'll cover iterators/enumerators on another chapter, for now, just think as a way to go over an structure
* we check the space, if there is a space, we return then the index of that char for the string
* otherwise, if there are no spaces, we return the size of the string...

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

The usage is quite stright forward, we now get first_word to receive a "read-only" reference of our string, and return the index of the first word.
After that, we could still use our string, since we only borrowed the read-only reference, but we then use clear to empty the string.

Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!

```rust
fn second_word(s: &String) -> (usize, usize) {
```

### String slices

A string slice is a reference to part of a String, and it looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

* Similar to a reference, but it refers to a range of he string, defined by [from..to]
* We can use ..= to define that is an inclusive range.

```rust
let s = String::from("hello world");

let hello = &s[0..=4];
let world = &s[6..=10];
```


In memory, it looks like this:

```
STACK          |     HEAP
               |
  s            |
NAME | VALUE   |  INDEX | VALUE
 ptr | --------|->   0  |  h
 len | 11      |     1  |  e
 cap | 11      |     2  |  l
               |     3  |  l
  word         |     4  |  o
NAME | VALUE   |     5  |
 ptr | --------|->   6  |  w
 len | 11      |     7  |  o
 cap | 11      |     8  |  r
                       ...
```

You can also use:
* `[..2]` to start from the 0
* `[2..]` to copy until the end
* `[..]` return a full range

We can the, rewrite our function to something like:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

By always using references, the return is tight to the content, leaving no room for dangling indexes...

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

Now, we can't mutate anymore, since we borrowed a read reference to first_word.

### String Literals Slices

A signature with `&str` to `&str` allows us to use both literals as `String`.

```rust
fn first_word(s: &String) -> &str {}
```

### String Slice as Parameters

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i]
        }
    }

    &s[..]
}
```

## Other Slices

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

The type is `&[i32]`.
