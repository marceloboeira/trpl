# Chapter 7

# Packages, Crates and Modules

* `Packages` are a Cargo feature that let you build, test, and share crates.
* `Crates` are a tree of modules that produce a library or executable.
* `Modules` and the use keyword let you control the scope and privacy of paths.
* A `path` is a way of naming an item such as a struct, function, or module.

## The Module System to Control Scope and Privacy

* Modules, a way to organize code and control the privacy of paths
* Paths, a way to name items
* `use` a keyword to bring a path into scope
* `pub`, a keyword to make items public
* Renaming items when bringing them into scope with the `as` keyword
* Using external packages
* Nested paths to clean up large `use` lists
* Using the glob operator to bring everything in a module into scope
* How to split modules into individual files

```rust
mod sound {
    fn guitar() {
        // Function body code goes here
    }
}
```

A function called guitar within a module called sound.

```rust
mod sound {
    mod instrument {
        mod woodwind {
            fn clarinet() {
                // Function body code goes here
            }
        }
    }

    mod voice {

    }
}
```

We have then, something like:

```
crate
 └── sound
      └── instrument
              └── woodwind
                   └── voice
```


## Paths for Referring to an Item in the Module Tree

* An absolute path starts from a crate root by using a crate name or a literal crate.
* A relative path starts from the current module and uses self, super, or an identifier in the current module.

```rust
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Fails to compile, because of privacy...

## Using the pub Keyword to Make Items Public

* All items (functions, methods, structs, enums, modules, annd constants) are private by default.
* You can use the pub keyword to make an item public.
* You aren't allowed to use private code defined in modules that are children of the current module.
* You are allowed to use any code defined in ancestor modules or the current module

```rust
mod sound {
    pub mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Then, finally:

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

# Starting Relative Paths with `super`

e.g.:

```rust

mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // Function body code goes here
}
```

Or even, sharing the same parent module...

```rust
mod sound {
    mod instrument {
        fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {
        // Function body code goes here
    }
}
```

# Using `pub` with `Structs` and `Enums`

```rust
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
```

## The `use` Keyword to Bring Paths into a Scope

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Or even:

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use self::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

## Idiomatic use Paths for Functions vs. Other Items

importing functions or values...

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument::clarinet;

fn main() {
    clarinet();
    clarinet();
    clarinet();
}
```

Use a HashMap:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Less explicitly:

```rust
use std::collections;

fn main() {
    let mut map = collections::HashMap::new();
    map.insert(1, 2);
}
```

One exception to the idiom is if the use statements would bring two items with the same name into scope, which isn't allowed.

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}
```

If instead we specified use `std::fmt::Result` and use `std::io::Result,` we’d have two `Result` types in the same scope and Rust wouldn’t know which one we meant when we used Result.

## Renaming Types Brought Into Scope with the as Keyword

Solving conflicts, or even renaming modules/types and such...

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}
```

## Using External Packages

```toml
[dependencies]
rand = "0.5.5"
```

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

##  Nested Paths for Cleaning Up Large use Lists

```rust
use std::cmp::Ordering;
use std::io;
```

We can instead, do something line:

```rust
use std::{cmp::Ordering, io};
```

```rust
use std::io;
use std::io::Write;
```

```rust
use std::io::{self, Write};

```


## Bringing All Public Definitions into Scope with the Glob Operator

```rust
use std::collections::*;
```

## Separating Modules into Different Files

Check `my-project/` to see how to do that.

```
src/
src/sound.rs
src/sound/instrument.rs
```
