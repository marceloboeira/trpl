# Chapter 15

## Smart Pointers

A pointer is a general concept for a variable that contains an address in memory.

This address refers to, or “points at,” some other data, its value is just the address of a memory on the Heap.

The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4.

References are indicated by the & symbol and borrow the value they point to.

They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of pointer we use most often.

We’ve already encountered a few smart pointers in this book, such as String and Vec<T> in Chapter 8, although we didn’t call them smart pointers at the time. Both these types count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with String ensuring its data will always be valid UTF-8).

Smart pointers are usually implemented using structs. The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers. The Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope. In this chapter, we’ll discuss both traits and demonstrate why they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won’t cover every existing smart pointer. Many libraries have their own smart pointers, and you can even write your own. We’ll cover the most common smart pointers in the standard library:

* Box<T> for allocating values on the heap
* Rc<T>, a reference counting type that enables multiple ownership
* Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
* In addition, we’ll cover the interior mutability pattern where an immutable type exposes an API for mutating an interior value. We’ll also discuss reference cycles: how they can leak memory and how to prevent them.

Let’s dive in!

## Using Box<T> to Point to Data on the Heap

The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. Refer to Chapter 4 to review the difference between the stack and the heap.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

* When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
* When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
* We’ll demonstrate the first situation in the “Enabling Recursive Types with Boxes” section. In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap. The third case is known as a trait object, and Chapter 17 devotes an entire section, “Using Trait Objects That Allow for Values of Different Types,” just to that topic. So what you learn here you’ll apply again in Chapter 17!

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

See `example01.rs`.

### Enabling Recursive Types with Boxes
At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

Let’s explore the cons list, which is a data type common in functional programming languages, as an example of a recursive type. The cons list type we’ll define is straightforward except for the recursion; therefore, the concepts in the example we’ll work with will be useful any time you get into more complex situations involving recursive types.

#### More Information About the Cons List

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

```
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ----- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

#### Computing the Size of a Non-Recursive Type

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a Message value, Rust goes through each of the variants to see which variant needs the most space. Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, and so forth. Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.

Contrast this with what happens when Rust tries to determine how much space a recursive type like the List enum needs. The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List. Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List. To figure out how much memory the List type needs, the compiler looks at the variants, starting with the Cons variant. The Cons variant holds a value of type i32 and a value of type List, and this process continues infinitely, as shown.

```
i32 Cons
  i32 Cons
    i32 Cons
      i32 Cons
        i32 Cons
          ...
```

### Using Box<T> to Get a Recursive Type with a Known Size

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

Now, `Cons` has a specific size, since it will need the i32 (constant) + the pointer for `Box<List>` (even tho is of itself...).

The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

## Treating Smart Pointers Like Regular References with the Deref Trait

Implementing the Deref trait allows you to customize the behavior of the dereference operator, * (as opposed to the multiplication or glob operator). By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

Note: there's one big difference between the MyBox<T> type we're about to build and the real Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.

### Following the Pointer to the Value with the Dereference Operator

See `example03.rs`.

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

*y is equal to the value of x, since y is the reference and * is dereferencing... ('fetching' its content).

But, if we try:

```rust
    assert_eq!(5, y);
```

We get:
```
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`
```

Since we trying to compare a value to a reference, the cool part is that it's on compile time.

### Using Box<T> Like a Reference

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

See `example04.rs`.

The only difference is that here we set y to be an instance of a box pointing to the value in x rather than a reference pointing to the value of x.
In the last assertion, we can use the dereference operator to follow the box’s pointer in the same way that we did when y was a reference. Next, we’ll explore what is special about Box<T> that enables us to use the dereference operator by defining our own box type.

### Defining Our Own Smart Pointer

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

```
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
     |
     14 |     assert_eq!(5, *y);
        |                   ^^
```

See `example05.rs`.

### Treating a Type Like a Reference by Implementing the Deref Trait

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

We implement Deref for MyBox. The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.

When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:

```
*(y.deref())
```

### Implicit Deref Coercions with Functions and Methods

Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion converts a reference to a type that implements Deref into a reference to a type that Deref can convert the original type into. Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

See `example06.rs`.

Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value. Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref. The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```
### How Deref Coercion Interacts with Mutability

Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

* From &T to &U when T: Deref<Target=U>
* From &mut T to &mut U when T: DerefMut<Target=U>
* From &mut T to &U when T: Deref<Target=U>

The first two cases are the same except for mutability. The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that there is only one immutable reference to that data, and the borrowing rules don’t guarantee that. Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.

## Running Code on Cleanup with the Drop Trait

The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the Drop trait on any type, and the code you specify can be used to release resources like files or network connections. We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For example, Box<T> customizes Drop to deallocate the space on the heap that the box points to.

In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

See `example07.rs`.

### Dropping a Value Early with std::mem::drop

Unfortunately, it’s not straightforward to disable the automatic drop functionality. Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically. Occasionally, however, you might want to clean up a value early. One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock to run so other code in the same scope can acquire the lock. Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

See `example08.rs`.

Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main. This would be a double free error because Rust would be trying to clean up the same value twice.

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

### Rc<T>, the Reference Counted Smart Pointer

In the majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.

To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting. The Rc<T> type keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

**Note that Rc<T> is only for use in single-threaded scenarios. When we discuss concurrency in Chapter 16, we’ll cover how to do reference counting in multithreaded programs.**

### Using Rc<T> to Share Data

```
b ---------|3|-|
           |a| |---> |5| |---> |10| |---> Nil
c ---------|5|-|
```

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

It fails since:
```
error[E0382]: use of moved value: `a`
  --> src/main.rs:13:30
   |
12 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
13 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
   = note: move occurs because `a` has type `List`, which does not implement
   the `Copy` trait
```

That's the scenario where `Rc<T>` is useful:

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

See `example09.rs`.

### Cloning an Rc<T> Increases the Reference Count

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

See `example10.rs`.

## RefCell<T> and the Interior Mutability Pattern

Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. We haven’t yet covered unsafe code; we will in Chapter 19. We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that. The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.

### Enforcing Borrowing Rules at Runtime with RefCell<T>

Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different from a type like Box<T>? Recall the borrowing rules you learned in Chapter 4:

* At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
* References must always be valid.

With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.

The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process, and there is no impact on runtime performance because all the analysis is completed beforehand. For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.

The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks. Static analysis, like the Rust compiler, is inherently conservative. Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem, which is beyond the scope of this book but is an interesting topic to research.

Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

* Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
* Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
* Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

### Interior Mutability: A Mutable Borrow to an Immutable Value

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```

### A Use Case for Interior Mutability: Mock Objects

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

The sent_messages field is now of type RefCell<Vec<String>> instead of Vec<String>. In the new function, we create a new RefCell<Vec<String>> instance around the empty vector.

For the implementation of the send method, the first parameter is still an immutable borrow of self, which matches the trait definition. We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>, which is the vector. Then we can call push on the mutable reference to the vector to keep track of the messages sent during the test.

The last change we have to make is in the assertion: to see how many items are in the inner vector, we call borrow on the RefCell<Vec<String>> to get an immutable reference to the vector.

Now that you’ve seen how to use RefCell<T>, let’s dig into how it works!

### Keeping Track of Borrows at Runtime with RefCell<T>

When creating immutable and mutable references, we use the & and &mut syntax, respectively. With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like regular references.

The catch is that if we break the rules, we get then a compile time error...

```rust
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

```
---- tests::it_sends_an_over_75_percent_warning_message stdout ----
    thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at
'already borrowed: BorrowMutError', src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Notice that the code panicked with the message already borrowed: BorrowMutError. This is how RefCell<T> handles violations of the borrowing rules at runtime.

### Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

```
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil)) }) })}) })})
```

See `example11.rs`.

## Reference Cycles Can Leak Memory

Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is not one of Rust’s guarantees in the same way that disallowing data races at compile time is, meaning memory leaks are memory safe in Rust. We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

### Creating a Reference Cycle

```rust
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

See `example12.rs`.

Basically, we point a reference of tail of A to be B, whereas be was already B->A->Nil, then it becomes, B->A->B->A->B->A.... until the stack explodes.

### Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

#### Creating a Tree Data Structure: a Node with Child Nodes

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

See `example13.rs`.

We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch. We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch. The reason is that leaf has no reference to branch and doesn’t know they’re related. We want leaf to know that branch is its parent. We’ll do that next.


#### Adding a Reference from a Child to Its Parent

To make the child node aware of its parent, we need to add a parent field to our Node struct definition. The trouble is in deciding what the type of parent should be. We know it can’t contain an Rc<T>, because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, which would cause their strong_count values to never be 0.

Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references!

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

```
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } }) }} }] }} })
```

See `example14.rs`.

### Visualizing Changes to strong_count and weak_count

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

See `example15.rs`.
