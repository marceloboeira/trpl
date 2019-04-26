# Chapter 19

## Unsafe Rust

All the code we’ve discussed so far has had Rust’s memory safety guarantees enforced at compile time. However, Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees: it’s called unsafe Rust and works just like regular Rust, but gives us extra superpowers.

nsafe Rust exists because, by nature, static analysis is conservative. When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to reject some valid programs rather than accept some invalid programs.

### Unsafe Superpowers

To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code. You can take four actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust. Those superpowers include the ability to:

* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait

#### Dereferencing a Raw Pointer

* Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup

### Calling an Unsafe Function or Method

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

#### Creating a Safe Abstraction over Unsafe Code

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

If we try to implement split_at_mut with safe code only, we face an issue:

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
```

We can't borrow it mut multiple times...
```
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
 -->
  |
6 |     (&mut slice[..mid],
  |           ----- first mutable borrow occurs here
7 |      &mut slice[mid..])
  |           ^^^^^ second mutable borrow occurs here
8 | }
  | - first borrow ends here
```

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```
See `example02.rs`.

slice::from_raw_parts_mut function takes a raw pointer and a length, and it creates a slice.

The function is unsafe because it takes a raw pointer and must trust that this pointer is valid.

```rust
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let slice: &[i32] = unsafe {
    slice::from_raw_parts_mut(r, 10000)
};
```

### Using extern Functions to Call External Code

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

See `example04.rs`.

#### Accessing or Modifying a Mutable Static Variable

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

See `example05.rs`.

### Implementing an Unsafe Trait

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

### When to Use Unsafe Code

Using unsafe to take one of the four actions (superpowers) just discussed isn’t wrong or even frowned upon. But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety. When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems if they occur.

## Advanced Lifetimes


* Lifetime subtyping: ensures that one lifetime outlives another lifetime
* Lifetime bounds: specifies a lifetime for a reference to a generic type
* Inference of trait object lifetimes: allows the compiler to infer trait object lifetimes and when they need to be specified

### Ensuring One Lifetime Outlives Another with Lifetime Subtyping

Lifetime subtyping specifies that one lifetime should outlive another lifetime. To explore lifetime subtyping, imagine we want to write a parser.

```rust
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

Compiling the code results in errors because Rust expects lifetime parameters on the string slice in Context and the reference to a Context in Parser.

he most straightforward way to do this is to use the same lifetime name everywhere:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

Next, let's try to use the context for a parsing function:

```rust
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

```
error[E0597]: borrowed value does not live long enough
  --> src/lib.rs:14:5
   |
14 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
15 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^

error[E0597]: `context` does not live long enough
  --> src/lib.rs:14:24
   |
14 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
15 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^
```

These errors state that the Parser instance that is created and the context parameter live only until the end of the parse_context function. But they both need to live for the entire lifetime of the function.

In other words, Parser and context need to outlive the entire function and be valid before the function starts as well as after it ends for all the references in this code to always be valid. The Parser we’re creating and the context parameter go out of scope at the end of the function, because parse_context takes ownership of context.

```rust
  fn parse(&self) -> Result<(), &str> {
```

```rust
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

```rust
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

which fails because:

```
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
 --> src/lib.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:1
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:1
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```
Rust doesn’t know of any relationship between 'c and 's. To be valid, the referenced data in Context with lifetime 's needs to be constrained to guarantee that it lives longer than the reference with lifetime 'c. If 's is not longer than 'c, the reference to Context might not be valid.

Finally,

```rust
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

See `example06.rs`.

### Lifetime Bounds on References to Generic Types

```rust
struct Ref<'a, T>(&'a T);
```

```
error[E0309]: the parameter type `T` may not live long enough
 --> src/lib.rs:1:19
   |
   1 | struct Ref<'a, T>(&'a T);
     |                   ^^^^^^
       |
         = help: consider adding an explicit lifetime bound `T: 'a`...
         note: ...so that the reference type `&'a T` does not outlive the data it points at
          --> src/lib.rs:1:19
            |
            1 | struct Ref<'a, T>(&'a T);
              |                   ^^^^^^''`'`'
```
### Inference of Trait Object Lifetimes

```rust
trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}
```

* The default lifetime of a trait object is 'static.
* With &'a Trait or &'a mut Trait, the default lifetime of the trait object is 'a.
* With a single T: 'a clause, the default lifetime of the trait object is 'a.
* With multiple clauses like T: 'a, there is no default lifetime; we must be explicit'''

## Advanced Traits

### Specifying Placeholder Types in Trait Definitions with Associated Types

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The type Item is a placeholder type, and the next method’s definition shows that it will return values of type Option<Self::Item>. Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
```

Why not simply?

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is that when using generics, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter.


### Default Generic Type Parameters and Operator Overloading

A great example of a situation where this technique is useful is with operator overloading. Operator overloading is customizing the behavior of an operator (such as +) in particular situations.

Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator. For example, in Listing 19-22 we overload the + operator to add two Point instances together


```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```
See `example08.rs`.

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```
This code should look generally familiar: a trait with one method and an associated type. The new part is RHS=Self: this syntax is called default type parameters. The RHS generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method. If we don’t specify a concrete type for RHS when we implement the Add trait, the type of RHS will default to Self, which will be the type we’re implementing Add on.

When we implemented Add for Point, we used the default for RHS because we wanted to add two Point instances. Let’s look at an example of implementing the Add trait where we want to customize the RHS type rather than using the default.

See `example09.rs`.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Rust allows multiple traits to have the same method and be implemented, or even for the struct it self to implement a method with the same name, but then we must be explicit when calling it...

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

See `example10.rs`.

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

See `example11.rs`.

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

### Using Supertraits to Require One Trait's Functionality Within Another Trait

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

To implement `OutlinePrint` is required of `Point` to also implement `fmt::Display`.

See `example12.rs`.


```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

See `example13.rs`.

## Advanced Types

### Using the Newtype Pattern for Type Safety and Abstraction

### Creating Type Synonyms with Type Aliases

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

Pretty much like haskell, no mistery here...

### The Never Type that Never Returns

```rust
fn bar() -> ! {
    // --snip--
}
```

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

However, this doesn't work:
```rust
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

The types must be the same, in order to make it work we use the `!` never type, look at how for instance, `unwrap` expands:

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

last, but not least, `loop`, which returns `!` since the loop could run forever....

```rust
print!("forever ");

loop {
    print!("and ever ");
}
```

### Dynamically Sized Types and the Sized Trait

```rust
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory

To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time.

```rust
fn generic<T>(t: T) {
    // --snip--
}
```

is actually treated as though we had written this:

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```


## Advanced Functions and Closures

### Function Pointers

* Function Pointer - Functions coerce to the type fn (with a lowercase f)
* Not to be confused with the `Fn` closure trait.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

See `example15.rs`.

Hability of sending functions as arguments:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```


We could simply:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

```rust
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> =
    (0u32..20)
    .map(Status::Value)
    .collect();
```

See `example16.rs`.

### Returning Closures

```rust
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```
Rust doesn’t know how much space it will need to store the closure. We saw a solution to this problem earlier. We can use a trait object:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
