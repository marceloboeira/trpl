# Chapter 10

## Generic Types, Traits, and Lifetimes

This chapter will cover code re-use with Generic, pattern definition with Traits and What are Lifetimes.

### Removing Duplication by Extracting a Function

First, an example of code re-use without generics:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

That finds the largest number on the number_list.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```
The second one finds the larges among 2 lists... each largest for each list.

But there is a lot of duplication... the boilerplate can be abstracted to a function.

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

1. Identify duplicate code.
1. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
1. Update the two instances of duplicated code to call the function instead.

## Generic Data Types

### In Function Definitions

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

Becomes:

```rust
fn largest<T>(list: &[T]) -> T {
    let mut l = list[0];

    for &item in list.iter() {
        if item > l {
            l = item;
        }
    }

    l
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

We need to implement `std::cmp::PartialOrd` fot `T`.


### In Struct Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

T represents any type, in this case, since we are not making huge use of it (specifiying behavior),

whereas, this wont' work:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

Since T can't assume 2 different types within Point, it must be a pair of x,y which carry the same type, e.g.: i32,i32, f32,f32 but not i32,f32.

We can also have Point with 2 different types, for that we can:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

### In Enum Definitions

```rust
enum Option<T> {
    Some(T),
    None,
}
```


Or even
```rust
enum Result<T,E> {
     Ok(T),
     Err(E),
}
```


### In Method Definitions


```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

More complex example:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition. Here, the generic parameters T and U are declared after impl, because they go with the struct definition. The generic parameters V and W are declared after fn mixup, because they’re only relevant to the method.

### Performance of Code Using Generics

> Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.

* Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compile


```rust
let integer = Some(5);
let float = Some(5.0);
```

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

The process of monomorphization makes Rust's generics extremely efficient at runtime.

## Traits: Defining Shared Behavior

A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

### Defining a Trait

* A trait tells the compiler about behavior that can be shared across types.
* We can use traits to define shared behavior in an abstract way.
* We can use trait bounds to specify that a generic can be any type that has certain behavior.
* It is similar to `interfaces` on other languages.


```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementing a Trait on a Type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

See `traits1.rs`.


### Default Implementations

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

See `traits2.rs`.

### Traits as arguments

See `traits3.rs`.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(tweet);
}
```

### Trait Bounds

The previous implementation of notify works for short examples, yet, it is a syntax sugar for:

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

This is called trait bound. The second version becomes less verbose over time... e.g.:

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {
// vs
pub fn notify<T: Summary>(item1: T, item2: T) {
```

### Specify multiple Traits with `+`

Sometimes, a function will required more than 1 trait to be implemented for its attribute...

```rust
pub fn notify(item: impl Summary + Display) {
```

Same is valid for trait bounds:
```rust
pub fn notify<T: Summary + Display>(item: T) {
```

### `where` clauses for clearer code

If you have multiple traits, the code looks a bit messed up:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

In order to scale that, we can use where clauses:

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

### Returning Traits

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

This signature says, “I’m going to return something that implements the Summary trait, but I’m not going to tell you the exact type.” In our case, we’re returning a Tweet, but the caller doesn’t know that.

However, this doesn't work:

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
```

This cannot work, due to restrictions around how impl Trait works. More on Chapter 17.

### Fixing the largest Function with Trait Bounds


```rust
fn largest<T>(list: &[T]) -> T {
    let mut l = list[0];

    for &item in list.iter() {
        if item > l {
            l = item;
        }
    }

    l
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

We need to update the signature to be like:
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
```

* PartialOrd - Adds behavior for ordering
* Copy - Allows values to be copied from memory rathar than pointed (we're talking only about stack values, not heap).


### Using Trait Bounds to Conditionally Implement Methods

When we implement a trait, we can implement to its extreme generic type, T, to the extreme specialized type e.g.: i32, or even, to types that are in between, for instance, any type that also implements PartialOrd.

e.g.:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

## Validating References with Lifetimes

Every reference in Rust has a lifetime, which is the scope where the reference is valid for.

Most of the times, lifetimes are implicit, and we must anotate to force this behavior to change.

### Preventing Dangling References with Lifetimes

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

This errors, with:

```
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:5
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

Mostly, because we try to assign a reference of a valua that lives in the inner-scope to the outer-scope, and in this case, `x` doesn't live long enough.

What happens if we try to access r after the scope of x is already done, **DANGLING REFERENCE**.

If Rust allowed this code to work, r would be referencing memory that was deallocated when x went out of scope, and anything we tried to do with r wouldn’t work correctly.

## The Borrow Checker

Rust has a borrow checker, that figures things as the above for us, preventing such mistakes.

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}
```

Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with `'b`. As you can see, the inner `'b` block is much smaller than the outer `'a` lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with a lifetime of `'b.` The program is rejected because `'b` is shorter than `'a:` the subject of the reference doesn’t live as long as the reference.

Let's fix the code and check again:

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means r can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

### Generic Lifetimes in Functions

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

Note that the `longest` takes `&str` because we don't want the function to take ownership of the values.

`&str` can accept both `String` and `str`.

### Lifetime Annotation Syntax

* Lifetime annotations don’t change how long any of the references live.
* Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. For example, let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a. The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a. The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.

### Lifetime Annotations in Function Signatures

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Note that the function handles both x and y as things with the same lifetime, `'a`. The lifetime of each attribute and the function itself must be equal or smaller than the other.

e.g.:

```rust
let string1 = String::from("long string is long");
{
  let string2 = String::from("xyz");
  let result = longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", result);
}
```

In this example, string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope.

Something like this doesn't work:

```rust
let string1 = String::from("long string is long");
let result;
{
  let string2 = String::from("xyz");
  result = longest(string1.as_str(), string2.as_str());
}
println!("The longest string is {}", result);
```

string2 *could* be the returning reference, but it doesn't live enough.

```
error[E0597]: `string2` does not live long enough
  --> src/main.rs:15:5
   |
14 |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
15 |     }
   |     ^ `string2` dropped here while still borrowed
16 |     println!("The longest string is {}", result);
17 | }
   | - borrowed value needs to live until here
```

### Thinking in Terms of Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

In this example, x is the only possibility of return, soon, y doesn't need to have a lifetime attribute.

When returning lifetime attributes, we must consider that the return has to be one of the attributes, because, otherwise, it won't have the same lifetime.

e.g.:

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

```
error[E0597]: `result` does not live long enough
 --> src/main.rs:3:5
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the
function body at 1:1...
 --> src/main.rs:1:1
  |
1 | / fn longest<'a>(x: &str, y: &str) -> &'a str {
2 | |     let result = String::from("really long string");
3 | |     result.as_str()
4 | | }
  | |_^
```

### Lifetime Annotations in Struct Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

This struct has one field, part, that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

The main function here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel. The data in novel exists before the ImportantExcerpt instance is created. In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.


#### Lifetime Elision


How does this work?

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

### Lifetime Annotations in Method Definitions

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### The Static Lifetime

```rust
let s: &'static str = "I have a static lifetime.";
```

`'static` denotates the duration of the entire program. All string literals have 'static lifetime.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together


```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
