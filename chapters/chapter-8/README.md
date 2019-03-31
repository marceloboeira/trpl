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

# Strings

## Types of String

* `str` - string slice, stack, compile time...
* `String` - String implementation, heap, runtime...
* More types - `OsString`, `OsStr`, `CString`, `CStr` borrowed/owned variants of String.

## Creating a new String

```rust
let mut s = String::new();
```

### From Literal to String

```rust
let data = "initial contents"; // literal
let s = data.to_string(); // String from literal s

// the method also works on a literal directly:
let s2 = "initial contents".to_string();
```

Also:

```rust
let s = String::from("initial contents");
```

### UTF8

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

## Updating a String

### Appending to a String with push_str and push

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```

See examples `example4.rs` and `example5.rs`.

```rust
let mut s = String::from("lo");
s.push('l');
```

* push_str - expects a string
* push - expects a char

### Concatenation with the + Operator or the format! Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

See `example6.rs`;

The reason `s1` is no longer valid after the addition and the reason we used a reference to `s2` has to do with the signature of the method that gets called when we use the `+` operator. The `+` operator uses the `add` method, whose signature looks something like this:

```rust
fn add(self, s: &str) -> String {
```

Note:

```
First, s2 has an &, meaning that we’re adding a reference of the second string to the first string because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together. But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add. So why does Listing 8-18 compile?

The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]. We’ll discuss deref coercion in more depth in Chapter 15. Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.
```


```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```
Same behavior as before for `s1`...

### Formating with macro

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

Trade-off is that `formating!` macro generates a new String, and just like `println!` it doens't take the ownership.

# Indexing into Strings

```rust
let s1 = String::from("hello");
let h = s1[0];
```

It doesn't work because...

### Internal Representation

String is an abstraction built on top of `Vec<u8>`.
```rust
let len = String::from("Hola").len();
```

This returns 4, becase each letter encoded in UTF8 takes 1 byte... However:

```rust
let len = String::from("Здравствуйте").len();
```

This returns 24, rather than the expected 12, because each `Unicode` scalar value in that string takes **2 bytes** of storage. Therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar value.

See `example7.rs`.
### Bytes and Scalar Values and Grapheme Clusters! Oh My!

There are 3 ways of looking at strings...
* as bytes
* scalar values
* grapheme clusters (the closest thing to what we would call letters).

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

```
['न', 'म', 'स', '्', 'त', 'े']
```

```
["न", "म", "स्", "ते"]
```

## Slicing

not a good idea because the sparsing is not necessairly linear, could be that some bytes take more than others to represent different chars.

## Iterating

### Chars

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

results in:

```
न
म
स
्
त
े
```

### Bytes
```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

```
224
164
// --snip--
165
135
```

But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
