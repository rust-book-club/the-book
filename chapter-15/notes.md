# Chapter 15: Smart Pointers

Rust references (`&`) are bare pointers, with no overhead.

_Smart_ pointers are pointers with additional metadata and capabilities.

For example, `String` and `Vec<T>` are both smart pointers, because they both own some data, and provide extra functionality wrt. that data.

Smart pointers are usually implemented using structs which implement the `Deref` and `Drop` traits.
- `Deref` allows an instance of the smart pointer struct to behave like a reference
- `Drop` allows you to define the code that's run when the pointer goes out of scope

"Smart pointers" is a general design pattern in Rust, and there are lots of them.

## Using `Box<T>` to Point to Data on the Heap

"The most straightforward smart pointer... boxes allow you to store data on the heap rather than the stack."

Use boxes when you
1. need a type of an exact size, but have a type whose size can't be known at compile time
2. have a large amount of data and want to transfer ownership, but don't want to copy everything
3. want to own a value that implements a particular trait, without caring what type it is

Case (1) will be covered in [Enabling Recursive Types with Boxes](https://rust-book.cs.brown.edu/ch15-01-box.html#enabling-recursive-types-with-boxes).

Case (2) will be covered in the next section.

Case (3) is a _trait object_ and will be covered in Chapter 17.

### Using a `Box<T>` to Store Data on the Heap

```rs
fn main() {
    let b = Box::new(5);   // store the value 5 on the heap
    println!("b = {}", b);
}                          // b and b's data are both deallocated here
```

### Enabling Recursive Types with Boxes

Recursive types can contain themselves and therefore have potentially infinite size.

Boxes help to sidestep this issue.

#### More Information About the Cons List

```rs
enum List {
    Cons(i32, List), // this won't compile
    Nil,
}
```

```rs
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
//   |
// 2 |     Cons(i32, Box<List>),
//   |               ++++    +
```

#### Computing the Size of a Non-Recursive Type

```rs
enum Message {                  // needs enough space to hold the largest of its variants (24 bytes)
    Quit,                       //   needs no space
    Move { x: i32, y: i32 },    //   needs enough space to hold two i32 values (64 bits or 8 bytes)
    Write(String),              //   needs enough space to hold a pointer, some metadata (24 bytes)
    ChangeColor(i32, i32, i32), //   needs enough space to hold three i32 values (12 bytes)
}
```

#### Using `Box<T>` to Get a Recursive Type with a Known Size

`Box<T>` is a pointer with a known size.

> _"Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another."_

```rs
enum List {
    Cons(i32, Box<List>), // needs <i32> + <usize> amount of space
    Nil,                  // needs no space
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

> _"Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities..."_

## Treating Smart Pointers Like Regular References with the `Deref` Trait

Implementing `Deref` allows dereferencing with `*`.

### Following the Pointer to the Value

```rs
fn main() {
    let x = 5;
    let y = &x; // y is a reference (a pointer) to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereferencing follows the pointer to the value
}
```

### Using `Box<T>` Like a Reference

```rs
fn main() {
    let x = 5;
    let y = Box::new(x); // x is copied; y is an instance of a Box with a pointer to this copied value

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### Defining Our Own Smart Pointer

"Let’s build a smart pointer similar to the `Box<T>` type..."

```rs
struct MyBox<T>(T); // tuple struct with one element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { // to match Box::new
        MyBox(x)
    }
}
```

### Treating a Type Like a Reference by Implementing the `Deref` Trait

```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // associated type (covered in Chapter 19)

    fn deref(&self) -> &Self::Target { // returns a reference &
        &self.0 // *myBox is syntactic sugar for *(myBox.deref())
    }
}
```

### Implicit Deref Coercions with Functions and Methods

> _"Deref coercion converts a reference to a type that implements the `Deref` trait into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`."_

Deref coercion lets us write

```rs
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

instead of

```rs
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

Rust will call `Deref::deref` as many times as necessary to get the correct type. There is no runtime penalty because this path is calculated at compile time.

### How Deref Coercion Interacts with Mutability

Implement `DerefMut` to override the `*` operator on mutable references.

Rust will convert `&T` to `&U`, `&mut T` to `&mut U`, and `&mut T` to `&U` using deref coercion, but it will not coerce `&T` to `&mut U`, because this could violate the borrowing rules.

## Running Code on Cleanup with the `Drop` Trait

`Drop` can be implemented on any type
- for example, files and network connections, which should be closed
- when a `Box<T>` is dropped, it will deallocate its space on the heap
- helps to prevent resource leaks!
- variables are dropped in the reverse order of their creation

```
// minimal example
struct Example {
    // impl
}

impl Drop for Example {
    fn drop(&mut self) {
        // do stuff here
    }
}
```

## Dropping a Value Early with `std::mem::drop`

Rust doesn't let you call `drop()` manually (`explicit destructor calls not allowed`), you have to use `std::mem::drop`

> _"Rust doesn’t let us call `drop` explicitly because Rust would still automatically call `drop` on the value at the end of `main`. This would cause a_ double free _error because Rust would be trying to clean up the same value twice."_

## `Rc<T>`, the Reference Counted Smart Pointer

When do we need multiple ownership? In graphs, linked lists, etc.

> _"We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect."_

`Rc<T>` is only for use in single-threaded scenarios. Multi-threaded scenarios are discussed in Chapter 16.

### Using `Rc<T>` to Share Data

We can use an `Rc<T>` instead of a `Box<T>` in this example

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Rc::clone() increases the number of references
    let c = Cons(4, Rc::clone(&a));
}
```

Use `Rc::clone(&a)` instead of `a.clone()` to visually distinguish that we are only increasing the reference count (which is fast), rather than copying a bunch of data like `clone()` does for most implementations.

### Cloning an `Rc<T>` Increases the Reference Count

Use `Rc::strong_count(&a)` to get the number of references to `a`.

(As opposed to `weak_count`, which we'll see in the next section.)

`Rc<T>` lets you have many _immutable_ references to some data.

What about multiple mutable references? We'll see `RefCell<T>` and _interior mutability_ in the next section.

## `RefCell<T>` and the Interior Mutability Pattern

_Interior mutability_ is a design pattern in Rust that
- allows you to mutate data, even when there are immutable references to that data
- uses `unsafe` code inside a data structure to bend Rust's usual borrowing / mutation rules

> _"Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us; we will discuss unsafe code more in Chapter 19."_

`RefCell<T>` is a type which follows the interior mutability pattern.

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`

With `Box<T>`, borrowing rules are enforced at compile time.
- if you break these rules, you'll get a compiler error

With `RefCell<T>`, borrowing rules are enforced _at runtime_.
- if you break these rules, your program will panic and exit

Rust is conservative and enforces borrowing rules at compile time by default.

But the compiler can't analyze all scenarios.

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

> _"Mutating the value inside an immutable value is the_ interior mutability _pattern."_

### Interior Mutability: A Mutable Borrow to an Immutable Value

```rs
fn main() {
    let x = 5;
    let y = &mut x; // error[E0596]: cannot borrow `x` as mutable
}
```

> _"However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code. Code outside the value’s methods would not be able to mutate the value."_

Borrowing rules are still checked, just at runtime instead of compile time.

#### A Use Case for Interior Mutability: Mock Objects

```rs
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) { // self is immutable to conform to interface
            self.sent_messages.push(String::from(message)); // push mutates
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

becomes

```rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // use a RefCell<T> instead
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]), // wrap the vec![]
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) { // borrow_mut() before the push()
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        // borrow() returns an immutable borrow
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

#### Keeping Track of Borrows at Runtime with `RefCell<T>`

`&` and `&mut` create immutable and mutable references to objects.

`borrow()` and `borrow_mut()` on `RefCell<T>` also create immutable and mutable references.
- `borrow()` returns a `Ref<T>`
- `borrow_mut()` returns a `RefMut<T>`
- `Ref<T>` and `RefMut<T>` both implement `Deref` and so can be treated like regular references

`RefCell<T>` keeps track of its own immutable and mutable references internally, and follows the regular Rust borrowing rules, only allowing you to have one mutable reference at a time.

So, our `MockMessenger` looks to the compiler like an immutable object, but we are allowing it to be mutable at runtime.

> _"...using `RefCell<T>` makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while you’re using it in a context where only immutable values are allowed."_

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

`Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data.

An `Rc<RefCell<T>>` would give multiple owners _mutable_ access.
- runtime borrowing rule checks prevent data races

Remember that this is all single-threaded. `Mutex<T>` is a thread-safe version of `RefCell<T>`, which we will discuss in Chapter 16.

## Reference Cycles Can Leak Memory

> _"...using `Rc<T>` and `RefCell<T>`... it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped."_

### Creating a Reference Cycle

```rs
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // b's tail is a

if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b); // now a's tail is b
}
```

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

Calling `Rc::downgrade` and passing a reference to the `Rc<T>` creates a _weak reference_, `Weak<T>`.

`Rc<T>` values have a `strong_count` and a `weak_count` of references, which are tracked separately.

`weak_count` doesn't need to be 0 for the `Rc<T>` instance to be cleaned up.

To turn a `Weak<T>` into a `T`, call its `upgrade` method, which will return a `Some<T>` if the value is still available (hasn't been cleaned up after its `strong_count` reached 0), and a `None` otherwise.

`RefCell<Weak<T>>` doesn't create a reference cycle in the way `RefCell<Rc<T>>` does, because weak references can be dropped at any time, unlike strong references, which are guaranteed to exist.

#### Creating a Tree Data Structure: a `Node` with Child Nodes

#### Adding a Reference from a Child to Its Parent

> _"...a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references!"_

```rs
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // this node only needs a weak reference to its parent
    children: RefCell<Vec<Rc<Node>>>, // this node's children are dropped when this node is dropped
}
```

> _"A node will be able to refer to its parent node but doesn’t own its parent."_

## Summary

We covered a lot this chapter: `Box<T>`, `Rc<T>`, `RefCell<T>`, interior mutability, reference cycles, `Deref` and `Drop`, smart pointers, and `Weak<T>`.

[The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) contains more information about creating smart pointers.

Next chapter: concurrency.

We're 3/4 of the way through the book at the end of Chapter 15!