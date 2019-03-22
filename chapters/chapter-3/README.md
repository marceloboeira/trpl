# Chapter 3

# Mutability

Example 1:

```rust
fn main() {
    -- before: let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

## Constants

* There is no type inference
* Constants can be declared in any scope (including global)
* May be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime

## Shadowing

By using let statements you can re-define a variable (previously defined).
Example 2:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

### What's the difference between mutability and shadowing?

With shadowing you can change the type of the variable, yet you must declare it properly.

Shadowing immutables also gives you the ability of prevent unwanted assignments.

**Example 3**

This works:
```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

This doesn't:
```rust
fn main() {
    let mux spaces = "   ";
    spaces = spaces.len();
}
```

# Data Types

* Every value in Rust is of a certain data type
* Statically Typed - rust must know the types of all variables at compile time
* The compiler usually infers what type we want to use based on the value
* Data type subsets: scalar and compound

## Scalar Types
> A scalar type represents a single value.

* Rust has four primary scalar types: `integers`, `floating-point numbers`, `Booleans`, and `characters`.

### Integer Types

```rust
u8 - Signed 8 bit
i16 - Unsinged 16 bit
i32 - Unsinged 32 bit
u64
```

* Carefull with overflows

#### Literals

```rust
Decimal	        98_222
Hex	            0xff
Octal	          0o77
Binary	        0b1111_0000
Byte (u8 only)	b'A'
```

### Floating Point Types

```rust
f32 - 32 bits floating point
f64 - 64 bits floating point
```


* It defaults to `f64`

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### Numerical Operations

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // multi-type operation
    let multi = 43 + 5.5;
}
```

Numerical operations are bound to types too, so:

```rust
error[E0277]: cannot add a float to an integer
  --> example4.rs:18:24
   |
18 |     let multi = 43 + 4.5;
   |                    ^ no implementation for `{integer} + {float}`
   |
   = help: the trait `std::ops::Add<{float}>` is not implemented for `{integer}`
```

### Boolean

Quite straight forward:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

* Booleans are one byte in size.

### Char

* The char literal is specified with single quotes, as opposed to string literals, which use double quotes.
* Rust’s char type represents a Unicode Scalar Value

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

## Compound Types
> Compound types can group multiple values into one type.

Rust has two primitive compound types: `tuples` and `arrays`.


### Tuples

Quite straight forward:

```rust
fn main() {
    // let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;

    // Direct access
    let five_hundred = tup.0; // As with most programming languages, the first index in a tuple is 0.
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of y is: {}", y);
}
```

### Arrays
> Arrays are useful when you want your data allocated on the stack rather than the heap

* An array isn’t as flexible as the vector type
* A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
* If you’re unsure whether to use an array or a vector, you should probably use a vector

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // [type;number]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // accessing elements
    let first = a[0];
    let second = a[1];
}
```

# Functions


* Entrypoint is `main()`
* You’ve also seen the fn keyword, which allows you to declare new functions
* Rust code uses **snake case** as the conventional style for function and variable names
* Rust doesn’t care where you define your functions, only that they’re defined somewhere

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Parameters

```rust
fn main() {
    another_function(5, 6);
}

// typed parameters
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

## Internal Statements

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

## Functions with Return Values

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

* `->` indicates a return type
* last statement is the return value, unless told otherwise (e.g.: early return - see `example10.rs`)
* return value can't have a semi-column

# Comments

```rust
// isolated comment

// multi
// line
// comment
fn main() {
    println!("Hello"); // same-line-as-code comment
}
```

# Control flow

## if statements

Quite straight forward, no need for parenthesis:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

The condition statement **must** be a boolean, and not any truthy/falsey thingy, as that concept doesn't exist on rust.

e.g.:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

This fails to compile, since 3 is not a bool.

### Multi if-statements

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### Conditional assignments

This works, but you need to ensure that the types are the same.
```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

For instance, this fails:
```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

## Repetition

Rust has 3 kinds of loops: `loop`, `while`, `for`.

### Loop

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

### Returning from loops

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

* You can use let statements with loops as well
* Loops can be stoped with break
* Break takes the return as a parameter

## while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

## for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

Can be replaced, with:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

With ranges:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```
