# Chapter 8

# Common Collections

* A `vector` allows you to store a variable number of values next to each other.
* A `string` is a collection of characters. We've mentioned the String type previously, but in this chapter we'll talk about it in depth.
* A `hash map` allows you to associate a value with a particular key. It's a particular implementation of the more general data structure called a map.

# Vectors

## Creating a New Vector

```rust
let v: Vec<i32> = Vec::new();
```

Vector uses Generics, so we can use them with any type.

`Vec<T>` where `T` is any type, including another `Vec<T>`.

There is also a macro, to facilitate:

```rust
let v = vec![1, 2, 3];
```

## Updating a Vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

## Dropping a Vector Drops Its Elements

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```

## Reading Elements of Vectors

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) { // returns a Option<&T>
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

Since vectors are on the stack, you cannot check all possible errors during compile time, therefore, something line this could happen:

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

Where we ask for a unexistent element of the vector, causing a `Index out of bounds` error, a Runtime error.

You cannot borrow mutables and mutate the vector itself, as you're not able for single values:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

See `example1.rs`.

## Iterating over the Values in a Vector

Read-only:

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

Mutation:

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

See `example2.rs`.

## Using an Enum to Store Multiple Types

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

See `example3.rs`.
