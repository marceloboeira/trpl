# Chapter 5

# Defining and Instantiating Structs

Similar to tuples, but with named arguments, non-positional.

e.g.:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Using a strucut:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

Example:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User {} logged in with {} email address", user1.username, user1.email);
}
```

### Mutable

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

See example2.rs.

### Default attributes

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

See example3.rs.

### Creating Instances From Other Instances With Struct Update Syntax

We can simplify this:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

With this:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

See `example4.rs`.

### Tuple Structs / Named Tuples

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
See `example5.rs`.

### Unit Like Structs

Empty tuples are useful when you have no data but the type itself. More to come.


### Ownership

We used the owned `String` type rather than the `&str` string slice type. We want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10.

Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

# An Example Program Using Structs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

* + Good names
* - No easy to scale
* - Coupled


## Refactoring with Tuples

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

* - Not good names
* ~ Better to scale (carries the attributes on a single, but positional unit)
* - Less coupled, but still coupled (positional)

## Refactoring with Structs: Adding More Meaning

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

* + Good names
* + Not coupled
* + Easier to scale (single unit, non-positional)

## Adding Useful Functionality with Derived Traits

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

```
error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
```

`println!` macro with `{}` expects the type to implement the trait `Display` from standard library, which all the primitives do.

Display doesn't assume anything, you have to implement it for most types that you create. Mainly, because you have other means to display values with assumptions. It's implemented for the primitives, because, for instance, 1 has not so many different ways of being displayed, whereas, our rectangle could be displayed in many different ways.

If you try:
```rust
println!("rect1 is {:?}", rect1);
```

You get a different error, since `{:?}` expects	the `Debug` trait to be implemented.

The main difference is that the Debug trait has a default implementation through an annotation...

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

Which by running, you get:

```
rect1 is Rectangle { width: 30, height: 50 }
```

And, if your structure grows, over time, you can even use:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

Which generates:

```
rect1 is Rectangle {
   width: 30,
   height: 50
}
```

More readable.

## Method Syntax

Our area function is very specific: it only computes the area of rectangles.

It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.

Methods are different from functions in that they're defined within the context of a struct and their first parameter is always self, which represents the instance of the struct the method is being called on.

### Defining Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

* `&self` a reference to an instance of the struct itself (that the call was triggered from).



### Where’s the `->` Operator?

In C and C++, two different operators are used for calling methods: you use `.` if you’re calling a method on the object directly and `->` if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, `object->something()` is similar to `(*object).something()`.

Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a feature called automatic **referencing** and **dereferencing**. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with `object.something()`, Rust automatically adds in `&,` `&mut`, or `*` so object matches the signature of the method. In other words, the following are the same:

```
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self.

Given the receiver and name of a method, Rust can figure out definitively whether the method is reading `(&self)`, mutating `(&mut self)`, or consuming `(self)`. The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

### Methods with More Parameters

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 }; let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Rect 1 w: {}, h: {}", rect1.width, rect1.height);
    println!("Rect 2 w: {}, h: {}", rect2.width, rect2.height);
    println!("Rect 3 w: {}, h: {}", rect3.width, rect3.height);
}
```

See `chapter-exercises/src/main6.rs`.

### Associated Functions

Basically, class functions, they are not called by an instance, yet the type itself. You must use `::` to call them.

e.g.:
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle::square(2);

    println!("Rect 1 w: {}, h: {}", rect1.width, rect1.height);
}
```

### Multiple Block

You can define new blocks of `impl` all around, even so, you don't **have/need** to.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
        );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
