# Chapter 2: Programming a Guessing Game

Rust _Prelude_: https://doc.rust-lang.org/std/prelude/index.html
 - similar to [Haskell's _Prelude_](https://hackage.haskell.org/package/base-4.17.0.0/docs/Prelude.html)

> _"Preludes can be seen as a pattern to make using multiple types more convenient. As such, you’ll find other preludes in the standard library, such as `std::io::prelude`. Various libraries in the Rust ecosystem may also define their own preludes."_

Some syntax
 - `let` creates a variable (immutable by default)
 - `let mut` creates a mutable variable
 - Single-line Rust comments begin with `//`, like in Java / Scala
 - Variables are assigned using `=`, like in Java / Scala
 - `String::new()` calls the _associated function_, `new()` on the `String` type

> _"[`String`](https://doc.rust-lang.org/std/string/struct.String.html) is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text."_

`&` indicates that a value is a _reference_, which we'll learn more about in Chapter 4

`Stdin::read_line()` returns a `Result` (simlar to a Scala `Try`) which is an _enumeration_ ("enum"). Each possible state of an enum is called a "variant".
 - Scala's `Try` can be a `Success` or a `Failure`
 - Rust's `Result` can be an `Ok` or an `Err`
 - `Result.expect()` works like `Try.get()`
 - Rust makes sure `Ok`s and `Err`s are handled, or the compiler gives a warning

__Note__: In Scala, we can write

```scala
s"this is a string ${42 + 42}"
```

and parse an expression inside an interpolated string. Rust can't do that. Only bare variables can go inside interpolated strings, expressions must appear afterward, like in a format string in Scala / Java:

```rust
println!("this is a string {}", 42 + 42)
```

See: https://stackoverflow.com/a/70504075/2925434

"binary crates" vs "library crates" in Rust -- like source code vs compiled class files in Java / Scala

Crates.io: https://crates.io/

`Cargo.lock` is a dependency lockfile. Update dependencies to compatible versions with `cargo update`. Upgrading e.g. `0.8.5` to `0.9.0` requires a rewrite of `Cargo.toml`. See: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html

Ranges in Rust: https://doc.rust-lang.org/std/ops/struct.Range.html

Use `cargo doc --open` to view all of the documentation for your current project's dependencies.

In Rust, `match` statements consist of "arms", and use exhaustive checking, just like in Scala.

Rust has a much richer variety of number types than Scala: `i32` (32-bit signed integers), `u32` (32-bit unsigned integers), `i64` (64-bit signed integers), etc. The default integer type is `i32`.

_Shadowing_ in Rust

```rust
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Similar to the nested scopes used in the `scala>` shell

```scala
scala> val thing: String = "forty-two"
val thing: String = forty-two

scala> val thing: Int = 42
val thing: Int = 42
```

Rust's `loop { ... }` creates an infinite loop until broken with the `break` keyword (jump back to the top of the `loop` using `continue`).

