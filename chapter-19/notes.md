# Chapter 19: Advanced Features

## Unsafe Rust

"...you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.”"

### Unsafe Superpowers

"You can take five actions in unsafe Rust that you can’t in safe Rust, which we call _unsafe superpowers_. Those superpowers include the ability to:"

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

> "...`unsafe` doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked. The `unsafe` keyword only gives you access to these five features that are then not checked by the compiler for memory safety. You’ll still get some degree of safety inside of an unsafe block."

Keep `unsafe` blocks as small as possible.

"...it’s best to enclose unsafe code within a safe abstraction and provide a safe API..."

### Dereferencing a Raw Pointer

Raw pointers are similar to references.

"As with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`, respectively. The asterisk isn’t the dereference operator; it’s part of the type name."

"In the context of raw pointers, _immutable_ means that the pointer can’t be directly assigned to after being dereferenced."

Rust raw pointers are []"exactly like pointers in C"](https://stanford-cs242.github.io/f19/lectures/07-2-smart-pointers.html#:~:text=Raw%20pointers,-Although%20not%20frequently&text=These%20are%20exactly%20like%20pointers,blocks%20for%20customized%20memory%20management).

Raw pointers

- are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- aren’t guaranteed to point to valid memory
- are allowed to be null
- don’t implement any automatic cleanup

```rs
    let mut num = 5;

    let r1 = &num as *const i32;   // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
```

"Notice that we don’t include the `unsafe` keyword in this code. We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block..."

Also, notice that we're creating two raw pointers to the same value, one mutable and one immutable. This is allowed with raw pointers, but not with references.

We _know_ that the above raw pointers are valid, because we created them directly from references guaranteed to be valid, but we could just as easily pick some random memory address

```rs
    let address = 0x012345usize;
    let r = address as *const i32;
```

"Creating a pointer does no harm; it’s only when we try to access the value that it points at that we might end up dealing with an invalid value."

```rs
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
```

Why would you ever want to use `unsafe` code?

- when interfacing with C
- when building up abstractions that the borrow checker doesn't understand

### Calling an Unsafe Function or Method

```rs
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```

#### Creating a Safe Abstraction over Unsafe Code

An example of a safe function that we cannot create using only safe Rust: `split_at_mut`.

```rs
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3); // split r at index 3

    assert_eq!(a, &mut [1, 2, 3]); // turn one mutable reference
    assert_eq!(b, &mut [4, 5, 6]); // ...into two mutable references
```

The closest we could get with safe Rust would be something like

```rs
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    // but this doesn't compile because
    //   "cannot borrow `*values` as mutable more than once at a time"
    //                        vvvvvv
    (&mut values[..mid], &mut values[mid..])
}
```

> "Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice. Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code."

A working implementation might look like

```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // get *mut i32 raw pointer

    assert!(mid <= len);

    unsafe {
        (
            // from_raw_parts_mut is unsafe because the ptr address might be invalid
            slice::from_raw_parts_mut(ptr, mid),
             // add is also unsafe because the offset address might be invalid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

#### Using `extern` Functions to Call External Code

Rust's `extern` keyword "facilitates the creation and use of a _Foreign Function Interface (FFI)_", which "is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions."

```rs
extern "C" { // this external function uses the C language Application Binary Interface (ABI)
    fn abs(input: i32) -> i32; // "the ABI defines how to call the function at the assembly level"
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Interoperability

- [C in Rust](https://docs.rust-embedded.org/book/interoperability/c-with-rust.html)
- [C++ in Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)#Interface_with_C_and_C++)

To make Rust code available to a C program, we would write

```rs
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Accessing or Modifying a Mutable Static Variable

We can define global variables in Rust with the `static` keyword

```rs
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

`static` variables always have a `'static` lifetime and are in `SCREAMING_SNAKE_CASE` by convention.

> "A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is _unsafe_."

```rs
static mut COUNTER: u32 = 0; // global mutable variable

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; // mutating a global variable is unsafe
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER); // even reading a global mutable variable is unsafe
    }
}
```

### Implementing an Unsafe Trait

"A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify."

```rs
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

`Sync` and `Send` are `unsafe trait`s, for example. We must manually verify that types implementing these traits can be safely accessed from multiple threads or passed across threads.

### Accessing Fields of a Union

Unions are primarily used to interface with unions in C code.

"Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance."

## Advanced Traits

### Specifying Placeholder Types in Trait Definitions with Associated Types

> [When to use generic type parameters vs. associated types?](https://stackoverflow.com/a/32065644/2925434) The short answer here is to prefer associated types unless you need to specify different behaviour for different parameterized types

```rs
pub trait Iterator {
    type Item; // <- associated type
    fn next(&mut self) -> Option<Self::Item>;
}
```

Implementing `Iterator` with associated types looks like

```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

If the `trait` used generics...

```rs
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

In this case, we could have

```rs
impl Iterator<String> for Counter
```

or

```rs
impl Iterator<u32> for Counter
```

or any number of implementations. The associated type, however, means that there can be only one implementation of `Iterator` for `Counter`.

> "...when a trait has a generic parameter, it can be implemented for a type multiple times... When we use the `next` method on `Counter`, we would have to provide type annotations to indicate which implementation of `Iterator` we want to use."

### Default Generic Type Parameters and Operator Overloading

Default generic type parameters look like `<PlaceholderType=ConcreteType>`

> "Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator."

For example, implementing `std::ops::Add` for a type defines the implementation of the `+` operator for that type

```rs
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point { // trait Add<Rhs=Self>
    type Output = Point;

    fn add(self, other: Point) -> Point { // fn add(self, rhs: Rhs) -> Self::Output;
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

An example where `Rhs` does not equal `Self`

```rs
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

> You’ll use default type parameters in two main ways:
> 
> - To extend a type without breaking existing code
> - To allow customization in specific cases most users won’t need

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Multiple traits define methods with the same signatures

```rs
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

The compiler infers that you want to use the most narrowly-scoped one

```rs
fn main() {
    let person = Human;
    person.fly(); // calls `Human::fly()`
}
```

If `Human` doesn't define its own `fly` method, the compiler complains

```rs
error[E0034]: multiple applicable items in scope
  --> src/main.rs:31:12
   |
31 |     person.fly();
   |            ^^^ multiple `fly` found
   |
note: candidate #1 is defined in an impl of the trait `Pilot` for the type `Human`
  --> src/main.rs:12:5
   |
12 |     fn fly(&self) {
   |     ^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `Wizard` for the type `Human`
  --> src/main.rs:18:5
   |
18 |     fn fly(&self) {
   |     ^^^^^^^^^^^^^
help: disambiguate the associated function for candidate #1
   |
31 |     Pilot::fly(&person);
   |     ~~~~~~~~~~~~~~~~~~~
help: disambiguate the associated function for candidate #2
   |
31 |     Wizard::fly(&person);
   |     ~~~~~~~~~~~~~~~~~~~~
```

Specify which method you mean by disambiguating

```rs
fn main() {
    let person = Human;
    Pilot::fly(&person);  // prints: This is your captain speaking.
    Wizard::fly(&person); // prints: Up!
    person.fly();         // prints: *waving arms furiously*
}
```

What if these functions are not methods, though? (What if they don't have a `self` parameter?)

```rs
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

We can call `Dog::baby_name()` directly, but the following example doesn't compile (`cannot infer type`)

```rs
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

So how can we call `Animal::baby_name()`?

We need to start with a concrete type which _implements_ `Animal`, like `Dog`

```rs
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

The Book calls this syntax (`<Dog as Animal>::method_name(...)`) "fully-qualified syntax". [Rust by Example](https://doc.rust-lang.org/rust-by-example/types/cast.html) calls this syntax (`Dog as Animal`) "casting".

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

[Supertraits vs. trait bounds](https://www.reddit.com/r/rust/comments/cqgyil/supertraits_vs_trait_bounds_newbie_question/) on Reddit

Recall: [trait bounds are used on generic parameters](https://doc.rust-lang.org/rust-by-example/generics/bounds.html) like

```rs
// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Supertraits are similar, but defined at the trait level, instead of at the value / method / function level

```rs
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) { // OutlinePrint provides this new method
        let output = self.to_string(); // output needs to be Display-able (to_string())
        println!("-> {} <-", output);
    }
}
```

### Using the Newtype Pattern to Implement External Traits on External Types

Remember [the orphan rule](https://rust-book.cs.brown.edu/ch10-02-traits.html#implementing-a-trait-on-a-type) from Chapter 10: we cannot implement a trait on a type if _both_ the trait and the type are external to our crate.

However, we can get around this restriction with the _newtype pattern_, where we wrap the external type in a tuple struct

> The term "newtype" comes from Haskell.

```rs
use std::fmt;

struct Wrapper(Vec<String>); // Vec<T> is defined outside our crate 

impl fmt::Display for Wrapper { // so is Display, but we can implement Display on Wrapper, which _is_ defined in this crate
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", ")) // and unwrap the wrapper here with self.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w); // Wrapper is Display-able
}
```

> "If we wanted the new type to have every method the inner type has, implementing the `Deref` trait... on the `Wrapper` to return the inner type would be a solution. If we don’t want the `Wrapper` type to have all the methods of the inner type—for example, to restrict the `Wrapper` type’s behavior—we would have to implement just the methods we do want manually."

## Advanced Types

### Using the Newtype Pattern for Type Safety and Abstraction

Newtypes can also be used

- like tagged types, e.g. `Meters(u32)` and `Millimeters(u32)` both wrap `u32`s, but represent different units
- to hide or abstract implementation details of the wrapped type by exposing a different API than the wrapped type

### Creating Type Synonyms with Type Aliases

```rs
    type Kilometers = i32; // Kilometers is a type alias for i32

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y); // i32 and Kilometers are the same type
```

No type-checking benefits from aliases like there are from newtypes. If a method expects a `Kilometers` and we pass an `i32`, we won't get any error.

> "The main use case for type synonyms is to reduce repetition."

```rs
    type Thunk = Box<dyn Fn() + Send + 'static>;

//  let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk                          = Box::new(|| println!("hi"));

//  fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

//  fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    fn returns_long_type() -> Thunk {
        // --snip--
    }
```

> "Type aliases are also commonly used with the `Result<T, E>` type for reducing repetition."

```rs
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
//  fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

//  fn flush(&mut self) -> std::result::Result<(), std::io::Error>;
    fn flush(&mut self) -> Result<()>;

//  fn write_all(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;

//  fn write_fmt(&mut self, fmt: fmt::Arguments) -> std::result::Result<(), std::io::Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

### The Never Type that Never Returns

`!` is the _empty type_ aka. the _never type_ (equivalent to Scala's `Nothing`)

> "Functions that return never are called _diverging functions_."

> "...expressions of type `!` can be coerced into any other type."

The `panic!` macro also has the type `!`

"One final expression that has the type `!` is a loop:"

```rs
    print!("forever ");

    loop {
        print!("and ever ");
    }
```

"Here, the loop never ends, so `!` is the value of the expression. However, this wouldn’t be true if we included a `break`, because the loop would terminate when it got to the `break`."

...okay, so?

### Dynamically Sized Types and the `Sized` Trait

`str` is a dynamically-sized type

```rs
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
```

> "Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory. If Rust allowed us to write this code, these two `str` values would need to take up the same amount of space. But they have different lengths: `s1` needs 12 bytes of storage and `s2` needs 15. This is why it’s not possible to create a variable holding a dynamically sized type."

We don't use `str` in code, though, we use string slices `&str`.

> "A `&str` is _two_ values: the address of the `str` and its length. As such, we can know the size of a `&str` value at compile time: it’s twice the length of a `usize`. That is, we always know the size of a `&str`, no matter how long the string it refers to is. In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind."

`trait`s are also dynamically-sized types, which is why we always refer to them behind a pointer when using them as trait objects, like `&dyn Trait`, `Box<dyn Trait>`, or `Rc<dyn Trait>`

> "To work with DSTs, Rust provides the `Sized` trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on `Sized` to every generic function. That is, a generic function definition like this:
> 
> ```rs
> fn generic<T>(t: T) {
>     // --snip--
> }
> ```
> 
> is actually treated as though we had written this:
> 
> ```rs
> fn generic<T: Sized>(t: T) {
>     // --snip--
> }
> ```
> 
> By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:
> 
> ```rs
> fn generic<T: ?Sized>(t: &T) {
>     // --snip--
> }
> ```
> 
> A trait bound on `?Sized` means “`T` may or may not be `Sized”` and this notation overrides the default that generic types must have a known size at compile time. The `?Trait` syntax with this meaning is only available for `Sized`, not any other traits.
> 
> Also note that we switched the type of the `t` parameter from `T` to `&T`. Because the type might not be `Sized`, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference."

## Advanced Functions and Closures

Passing closures to functions is different from passing regular functions to functions.

["How do you pass a Rust function as a parameter?"](https://stackoverflow.com/a/36390748/2925434)

["using impl vs dyn vs nothing for function as parameter"](https://www.reddit.com/r/rust/comments/eqmn0o/comment/feuaxbm/?utm_source=share&utm_medium=web2x&context=3)

`fn` function pointer type vs. `Fn` closure trait

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
```

`fn` implement all three closure traits: `Fn`, `FnMut`, and `FnOnce`, so

1. you can always pass a `fn` as an argument for a function which expects a closure
2. you should write functions using a generic type and one of the closure traits, to achieve this flexibility

Can't do (2) when interfacing with C, though, because C doesn't have closures.

Passing a closure vs. passing a function pointer

```rs
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // vs.
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
```

The `ToString` `trait` is implemented for any type which implements `Display`.

Enum variants also have corresponding initializer functions, which can be used as function pointers, like so

```rs
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

### Returning Closures

You can't return closures directly, because their size is not known at compile time

```rs
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

Instead, we have to use _trait objects_ again, which we saw in Chapter 17

```rs
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## Macros

There are two main types of macros

1. _declarative_ macros, defined with the `macro_rules!` macro (like `vec!`, `println!`, etc.)
2. _procedural_ macros, of which there are three kinds
    1. custom `#[derive]` macros which specify code added with the `derive` attribute on structs and enums
    2. "Attribute-like macros that define custom attributes usable on any item"
    3. "Function-like macros that look like function calls but operate on the tokens specified as their argument"

### The Difference Between Macros and Functions

"...macros are a way of writing code that writes other code, which is known as _metaprogramming_."

**Note:** `derive` attribute is discussed in Appendix C

Why macros over functions?

- macros can reduce the amount of code you have to write and maintain
- macros can take variable numbers or types of parameters, functions cannot
- macros are expanded before the compiler reads the code, so they can implement traits on types, etc.

Why functions over macros?

- macro definitions are more complex, and so more difficult to read, understand, and maintain
- macros must be defined or brought into scope before they are called, unlike functions which can be defined / called anywhere

### Declarative Macros with `macro_rules!` for General Metaprogramming

"The most widely used form of macros in Rust..."

"declarative macros" aka. "macros by example", "`macro_rules!` macros", or just "macros"

```rs
#[macro_export] // this macro should be made available whenever the crate in which the macro is defined is brought into scope
macro_rules! vec { // macro_rules! keyword, followed by the name of the macro without an exclamation point

    // structure of the macro body is similar to the structure of a match expression
    // pattern => { code to execute if pattern matches }

    // macro patterns are matched against Rust code structure rather than values
    // https://doc.rust-lang.org/reference/macros-by-example.html

    ( $( $x:expr ),* ) => { // variable x is bound to any Rust language expression
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }; // only one pattern, so anything which doesn't match results in an error
}
```

So

```rs
let v: Vec<u32> = vec![1, 2, 3];
```

is expanded to

```rs
let v: Vec<u32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};
```

This is like learning a whole separate programming language.

[The Little Book of Rust Macros](https://veykril.github.io/tlborm/)

### Procedural Macros for Generating Code from Attributes

Declarative macros match code against patterns and replace code with other code.

Procedural macros take some code as input, perform some operations on that code, and return new code.

Procedural macros are procedures while declarative macros are... expressions?

Three kinds of procedural macros

1. custom derive
2. attribute-like
3. function-like

> "When creating procedural macros, the definitions must reside in their own crate with a special crate type. This is for complex technical reasons that we hope to eliminate in the future."

```rs
use proc_macro; // defines the TokenStream type

#[some_attribute] // define a new procedural macro using the "some_attribute" macro variety; this is the "kind" of the macro
pub fn some_name(input: TokenStream) -> TokenStream { // the function that defines the procedural macro, named "some_name"
} // function must be of type (TokenStream) -> TokenStream
```

> (from the end-of-chapter quiz) "Procedural macros are the only way to create a custom derive. Procedural macros are also useful when you need code to analyze the macro user's syntax --- declarative macros only permit shuffling around the input, not e.g. computing its size. Declarative macros can generate variable-length sequences of code, and can wrap/produce items and not just expressions."

### How to Write a Custom `derive` Macro

> See `hello_macro` and `pancakes` crates in this directory.

`hello_macro` crate defines a `HelloMacro` `trait` with an associated function `hello_macro`, as well as a procedural macro which allows users to `#[derive(HelloMacro)]` to get a default implementation of `hello_macro` on their type, like

```rs
// pancakes/src/main.rs
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro(); // should print: Hello, Macro! My name is Pancakes!
}
```

> "Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime."

> "At the time of this writing, procedural macros need to be in their own crate. Eventually, this restriction might be lifted. The convention for structuring crates and macro crates is as follows: for a crate named `foo`, a custom derive procedural macro crate is called `foo_derive`."

```rs
// hello_macro/src/lib.rs
pub trait HelloMacro {
    fn hello_macro();
}
```

```rs
// hello_macro/hello_macro_derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

`syn::parse` returns something like the following for the above example

```rs
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

### Attribute-like macros

`derive` is an attribute. Attribute-like macros allow you to define _new_ attributes.

Attribute-like macros are more flexible, they can be applied to functions. `derive` only works for `struct`s and `enum`s

```rs
#[route(GET, "/")]
fn index() {
```

Attribute-like macros take two `TokenStreams`, which represent the values passed to the parameter list after the name of the macro (in the above case, that's `GET, "/"`), and the function below as well as its parameters and body

```rs
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

> "Other than that, attribute-like macros work the same way as custom derive macros: you create a crate with the `proc-macro` crate type and implement a function that generates the code you want!"

### Function-like macros

Function-like macros are similar to declarative macros except

- they use `!()` instead of `![]`
- they aren't required to use the `match`-like syntax that declarative macros use

"""

Function-like macros take a `TokenStream` parameter and their definition manipulates that `TokenStream` using Rust code as the other two types of procedural macros do. An example of a function-like macro is an `sql!` macro that might be called like so:

```rs
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

```rs
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

"""

