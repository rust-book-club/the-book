# Chapter 10: Generic Types, Traits, and Lifetimes

1. generic types
2. _"Then you’ll learn how to use traits to define behavior in a generic way"_
3. _"Finally, we’ll discuss lifetimes: a variety of generics that give the compiler information about how references relate to each other."_

Is (2) like structural typing?

(3) -- lifetimes are a kind of generic??

## Generic Data Types

struct example

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

method example

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

// constraints allowed
// "You cannot simultaneously implement specific and generic methods of the same name this way"
// "Rust does not have inheritance-like mechanisms for specializing methods as you might find in an
//  object-oriented language, with one exception (default trait methods) discussed in the next section."
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

There is no runtime cost to generics because Rust _monomorphizes_ generic types at compile time.

Function example

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

> _"unlike languages like Java where all objects have a set of core methods like [`Object.toString()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Object.html#toString()), there are no core methods in Rust. Without restrictions, a generic type `T` has no capabilities: it cannot be printed, cloned, or mutated (although it can be dropped)."_

This means even something like `fn f<T>(t: T) { println!("{t}"); }` won't compile.

## Traits: Defining Shared Behavior

### Implementing a Trait on a Type

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

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

They seem to behave generally like Scala's `trait`s or Java's `interface`s.

> _"Now that the library has implemented the `Summary` trait on `NewsArticle` and `Tweet`, users of the crate can call the trait methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods. The only difference is that the user must bring the trait into scope as well as the types."_

This is the behaviour we saw _so long ago_ where a trait had to be `use`d but it wasn't explained why...

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

...which kind of makes sense now. The trait `Summary` defines the method `summarize()` which is implemented for `Tweet`.

To implement a trait on a type, at least one of the trait or the type must be local to our crate. Otherwise, implementing an external trait on an external type could result in issues when people pull in _our_ library and also _some other library_ which implement the same external trait on the same external type.

> _"This restriction is part of a property called_ coherence, _and more specifically_ the orphan rule, _so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa."_

### Default Implementations

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

Nothing special needs to be done for `Tweet`, though, because "the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation."

Trait methods can call other trait methods, like in Scala

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

> _"Note that it isn’t possible to call the default implementation from an overriding implementation of that same method."

...so no `super` calls like in Scala / Java.

### Traits as Parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### Trait Bound Syntax

The above is shorthand for the more expressive _trait bound syntax_

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

> _"Using `impl Trait` is appropriate if we want this function to allow `item1` and `item2` to have different types (as long as both types implement `Summary`). If we want to force both parameters to have the same type, however, we must use a trait bound, like this:"_

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

#### Specifying Multiple Trait Bounds with the `+` Syntax

(Available in Scala 3 as [intersection types](https://www.baeldung.com/scala/intersection-types-scala-3).)

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

or

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

#### Clearer Trait Bounds with `where` Clauses

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

is equivalent to

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

### Returning Types that Implement Traits

> _"We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait, as shown here:"_

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

However...

> _"Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler. We’ll cover how to write a function with this behavior in the [“Using Trait Objects That Allow for Values of Different Types”](https://rust-book.cs.brown.edu/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) section of Chapter 17."_

### Using Trait Bounds to Conditionally Implement Methods

Oh now we're getting complex...

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// new() is available for Pair<T> of with any type <T>
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// but cmp_display() is only avalable for T which implement Display and PartialOrd
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

> _"We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called_ blanket implementations _and are extensively used in the Rust standard library."_

for example

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

Any type `T` which implements `Display` also implements `ToString` automatically.

## Validating References with Lifetimes

Lifetimes...
- are another kind of generic
- ensure that references are valid for as long as we need them to be
- defined for every reference in Rust
- are the scope for which a reference is valid

> _"We only must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid."_

### Preventing Dangling References with Lifetimes

> _"The main aim of lifetimes is to prevent_ dangling references..."

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
        //  ^^ borrowed value does not live long enough
    }
//  - `x` dropped here while still borrowed

    println!("r: {}", r);
}
```

### The Borrow Checker

```rust
fn main() {
    let r;                // ---------+-- 'a is the lifetime of r
                          //          |   
    {                     //          |
        let x = 5;        // -+-- 'b  |  'b is the lifetime of x
        r = &x;           //  |       |
    }                     // -+       |  r cannot reference x because r's
                          //          |  lifetime is longer than x's lifetime
    println!("r: {}", r); //          |
}                         // ---------+
```

Rearranging the code makes the subject of the reference live longer than the reference itself, and so the code will compile

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

### Generic Lifetimes in Functions

