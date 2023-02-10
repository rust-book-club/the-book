# Chapter 9: Error Handling

Recoverable vs unrecoverable errors
- `Result<T, E>` for recoverable errors
- `panic!` for unrecoverable ones

## Unrecoverable Errors with `panic!`

By default, when Rust `panic`s, it unwinds the stack, cleans up data, etc.

This behavior can be overridden by seeing `panic = 'abort'` in `Cargo.toml`.

### Using a `panic!` Backtrace

`run with RUST_BACKTRACE=1 environment variable to display a backtrace`

> _"Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote."_

You shouldn't catch `panic`s, you should just let the program crash:

> _"A panic should not be used to communicate failure within the program. The default assumption is that caller functions will not try to catch panics."_

## Recoverable Errors with `Result`

`Result` is to Rust as `Try` is to Scala

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result`, `Ok`, and `Err` are all brought into scope by Rust's prelude, just like `Option`, `Some`, and `None`.

### Matching on Different Errors

Handle different errors in different ways with nested `match` expressions

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

Here, the book talks about using `unwrap_or_else` (like Scala's `getOrElse`) to avoid the nested `match` expressions, but that doesn't seem any clearer to me. What _would_ be nice is something like Scala's _for comprehensions_ in Rust. Turns out, [there's a crate for that](https://docs.rs/map_for/latest/map_for/).

### Shortcuts for Panic on Error: `unwrap` and `expect`

Calling `unwrap` on a `Result` is like calling `get` on a `Try` in Scala -- it returns the result if it's `Ok` (a `Success`) and `panic`s (throws an Exception) if it's an `Err` (a `Failure`).

`expect` is like `unwrap` but it takes a String message that can explain _why_ we expect that `Result` to be `Ok`. The message is printed if the `Result` is not `Ok`.

> _"In production-quality code, most Rustaceans choose `expect` rather than `unwrap` and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging."_

### Propagating Errors

Return an error from a function with a return type of `-> Result<T, E>` and let the caller of the function deal with the error.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

#### A Shortcut for Propagating Errors: the `?` Operator

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

If `x` is an `Ok` then it is unwrapped; if it's an `Err` then the function returns immediately with that `Err` result.

> _"There is a difference between what the `match` expression from Listing 9-6 does and what the `?` operator does: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another. When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons."_

> *"For example, we could change the `read_username_from_file` function in Listing 9-7 to return a custom error type named `OurError` that we define. If we also define `impl From<io::Error>` for `OurError` to construct an instance of `OurError` from an `io::Error`, then the `?` operator calls in the body of `read_username_from_file` will call `from` and convert the error types without needing to add any more code to the function."*

`?`s can be chained. An even simpler implementation of the above could be

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

or even just

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### Where The `?` Operator Can Be Used

> _"...we’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`."_

Main can return `()` but also `Result<(), E>`.

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

> _"The `Box<dyn Error>` type is a_ trait object _, which we’ll talk about in the [“Using Trait Objects that Allow for Values of Different Types”](https://rust-book.cs.brown.edu/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) section in Chapter 17. For now, you can read `Box<dyn Error>` to mean “any kind of error.”"_

Rust returns `0` when a program has sucessfully run, and other nonzero integers, compatible with similar C return types, when a program has not completed successfully.

> _"The `main` function may return any types that implement [the `std::process::Termination` trait](https://doc.rust-lang.org/std/process/trait.Termination.html), which contains a function `report` that returns an `ExitCode` Consult the standard library documentation for more information on implementing the `Termination` trait for your own types."_

## To `panic!` or Not to `panic!`

`panic!` when it's impossible to recover from some problem, or when the program is in a bad state

return `Result` when the caller of the function should decide what to do

> _"In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a `Result`. Let’s explore why, then discuss situations in which the compiler can’t tell that failure is impossible, but you as a human can."_

### Examples, Prototype Code, and Tests

- in examples: proper error handling can make the focus less clear
- in prototype code: panic at first, and write error handling later
- in tests: `panic!` is how a test is marked as a failure

### Cases in Which You Have More Information Than the Compiler

For example

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

### Guidelines for Error Handling

`panic!` when code ends up in a _bad state_, encountering an error which could result in _insecure or harmful_ operations.

> _"...when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call"_

### Creating Custom Types for Validation

For example

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

> _"A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a `Guess` rather than an `i32` and wouldn’t need to do any additional checks in its body."_
