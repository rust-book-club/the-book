# Chapter 13: Functional Language Features: Iterators and Closures

This chapter will cover: closures and iterators

We've already covered other functional features: pattern matching, enums, etc.

## Closures: Anonymous Functions that Capture Their Environment

> _"Unlike functions, closures can capture values from the scope in which they’re defined."_

### Capturing the Environment with Closures

> *"The [`unwrap_or_else` method on `Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) is defined by the standard library. It takes one argument: a closure without any arguments that returns a value `T`..."*

### Closure Type Inference and Annotation

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

> _"The `add_one_v3` and `add_one_v4` lines require the closures to be evaluated to be able to compile because the types will be inferred from their usage. This is similar to `let v = Vec::new();` needing either type annotations or values of some type to be inserted into the `Vec` for Rust to be able to infer the type."_

Different syntax to Scala, of course:

```scala
def add_one_v1               (x: Int): Int = { x + 1 }
val add_one_v2: Int => Int =  x           => { x + 1 }
val add_one_v3             = (x: Int)     => { x + 1 }
val add_one_v4             = (x: Int)     =>   x + 1
```

A Rust closure with no parameters looks like `{ || doSomething() }`.

> *"`let f = |_| (); // sometimes called the "toilet closure"`"*

`f` (above) causes any argument passed to it to immediately be dropped, similar to [`std::mem::drop`](https://doc.rust-lang.org/std/mem/fn.drop.html).

### Capturing References or Moving Ownership

Closures infer type based on usage, but also infer borrowing rules.

If a closure takes an argument and prints it, Rust will infer that that argument is to be borrowed immutably (rather than taking ownership, or borrowing mutably), because that's all that's needed to print the value.

If a closure takes a vector argument and adds an element to it, Rust will infer that that argument is to be borrowed mutably.

You can force a closure to take ownership of a value which would otherwise only be borrowed by using the `move` keyword before the parameter list. "This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread."

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

### Moving Captured Values Out of Closures and the `Fn` Traits

- `FnOnce` closures
   - can be called at least once
- `FnMut` closures
   - capture values from their environment
      - don't move captured values out of their body
      - might mutate the captured values
   - can be called more than once
   - are also `FnOnce` closures
- `Fn` closures
   - might capture values from their environment
      - don't move captured values out of their body
      - don't mutate the captured values
   - can be called more than once
   - are also `FnOnce` closures

If a closure moves the captured value(s) out of its body, then it is a `FnOnce` closure which can be called at most once (because it doesn't retain the captured values).

The definition of `Option#unwrap_or_else` uses a `FnOnce` trait bound

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Similar to Scala, we can use a reference to a function instead of an explicit closure in a method like `unwrap_or_else`. For example, `unwrap_or_else(Vec::new)`.

This is an amazing compiler message (in the contrived example to show that `FnOnce` cannot be used in place of `FnMut`)

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:18:30
   |
15 |       let value = String::from("by key called");
   |           ----- captured outer variable
16 | 
17 |       list.sort_by_key(|r| {
   |  ______________________-
18 | |         sort_operations.push(value);
   | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
19 | |         r.width
20 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` due to previous error
```

### Closures Must Name Captured Lifetimes

```rust
fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    move || s_ref.to_string()
}
```

`make_a_cloner`, above, is invalid, because

```rust
fn main() {
    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own); // cloner closes over s_own
    drop(s_own); // s_own is freed
    cloner(); // cloner still has a reference to s_own ("use after free")
}
```

New diagrams -- need to reread Chapter 4!

I feel like I need to reread this section a few times to really absorb it, maybe after rereading Chapter 4.

## Processing a Series of Items with Iterators

Iterators in Rust

```rust
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter { // `for` takes ownership of the iterator
                         //   mutation done "behind the scenes"
        println!("Got: {}", val);
    }
```

### The `Iterator` Trait and the `next` Method

```rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // .next() mutates the iterator

        assert_eq!(v1_iter.next(), Some(&1)); // values are immutable references
        assert_eq!(v1_iter.next(), Some(&2)); // iter() produces an iterator over immut refs
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);     // iterator is now fully consumed
    }
```

To let the iterator take _ownership_ of the values, use `into_iter()` instead of `iter()`. To let the iterator take mutable references to each value, use `iter_mut()` instead of `iter()`.

### Methods that Consume the Iterator

`sum()`, for example

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // iterator is now consumed

        assert_eq!(total, 6);
    }
```

### Methods that Produce Other Iterators

> "Iterator adaptors _are methods defined on the `Iterator` trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator."_

`map()`, for example

```rust
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);
```

`collect()` will consume the above iterator (which is lazy, remember)

```rs
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
```

### Using Closures that Capture Their Environment

> _"Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment."_

`filter()`, for example

```rs
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

## Improving Our I/O Project

### Removing a `clone` Using an Iterator

#### Using the Returned Iterator Directly

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // --snip--
}
```

becomes

```rust
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // --snip--
}
```

### Using `Iterator` Trait Methods Instead of Indexing

```rust
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // --snip--
    }
}
```

becomes

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // --snip--
    }
}
```

### Making Code Clearer with Iterator Adaptors

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

becomes

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

## Comparing Performance: Loops vs. Iterators

> _"Iterators are one of Rust’s_ zero-cost abstractions, _by which we mean using the abstraction imposes no additional runtime overhead."_

Rust's compiler _unrolls_ loops, skips bounds checks when applicable, and just generally writes assembly code that's as performant as possible, even when using iterators and other FP constructs.
