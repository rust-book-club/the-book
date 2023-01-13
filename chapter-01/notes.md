# Chapter 0: Introduction

Open The Book locally with the command `rustup docs --book`

Experimental, interactive version of The Book: https://rust-book.cs.brown.edu/

Note that the free online book is the same text as https://nostarch.com/rust

## Introduction

"High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that 
conflict."

Tools: `cargo` dependency manager and build tool, `rustfmt` formatting tool, Rust Language Server which powers IDEs

What is Rust used for? "...command line tools, web services, DevOps tooling, embedded devices, audio and video analysis and 
transcoding, cryptocurrencies, bioinformatics, search engines, Internet of Things applications, machine learning, and even 
major parts of the Firefox web browser." Not to mention your standard low-level systems programming, plus big shiny projects 
like the Rust-to-webassembly web app, Figma.

Rust strives for "zero-cost abstractions"

### How to Use This Book

#### Chapters, and what they cover

1. "...how to install Rust, how to write a 'Hello, world!' program, and how to use Cargo, Rust’s package manager and build 
tool."

2. "...a hands-on introduction to writing a program in Rust, having you build up a number guessing game. Here we cover 
concepts at a high level, and later chapters will provide additional detail. If you want to get your hands dirty right away, 
Chapter 2 is the place for that."

3. "...Rust features that are similar to those of other programming languages..."

4. "...Rust's ownership system."

5. "...structs and methods..."

6. "...enums, `match` expressions, and the `if let` control flow construct."

7. "...Rust's module system and about privacy rules for organizing your code and its public Application Programming Interface 
(API)."

8. "...some common collection data structures that the standard library provides, such as vectors, strings, and hash maps."

9. "...Rust's error-handling philosophy and techniques."

10. "...generics, traits, and lifetimes, which give you the power to define code that applies to multiple types."

11. "...all about testing, which even with Rust's safety guarantees is necessary to ensure your program’s logic is correct."

12. "...we'll build our own implementation of a subset of functionality from the `grep` command line tool that searches for 
text within files."

13. "...closures and iterators: features of Rust that come from functional programming languages."

14. "...we'll examine Cargo in more depth and talk about best practices for sharing your libraries with others."

15. "...smart pointers that the standard library provides and the traits that enable their functionality."

16. "...different models of concurrent programming and talk about how Rust helps you to program in multiple threads 
fearlessly."

17. "...how Rust idioms compare to object-oriented programming principles you might be familiar with."

18. "...a reference on patterns and pattern matching, which are powerful ways of expressing ideas throughout Rust programs."

19. "...a smorgasbord of advanced topics of interest, including unsafe Rust, macros, and more about lifetimes, traits, types, 
functions, and closures."

20. "In Chapter 20, we’ll complete a project in which we’ll implement a low-level multithreaded web server!"

#### Appendices

A: Rust’s keywords

B: Rust’s operators and symbols

C: derivable traits provided by the standard library

D: some useful development tools

E: Rust editions

F: translations

G: how Rust is made and what nightly Rust is

---

The crab's name is Ferris

Source code of The Book: https://github.com/rust-lang/book/tree/main/src

---

# Chapter 1: Getting Started

## Installation

`rustup`: command line tool for managing Rust versions and associated tools

Other Rust installation methods
 - https://forge.rust-lang.org/infra/other-installation-methods.html
 - if you want to use `homebrew`, etc.

To update Rust: `rustup update`

Rust documentation: `rustup doc`

## Hello, World!

Appendix D has some cool development tools
 - https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html
 - `rustfmt` to maintain a consistent code style
 - `rustfix` to automatically fix common style mistakes
 - `clippy` to analyze code and fix more subtle / complex mistakes

`clippy`, in particular, looks really cool. An example they give is using the constant `3.1415` to represent pi in some code, and `clippy` is able to see that it's quite close to `f{32, 64}::consts::PI`, and recommends using that constant directly instead.

Rust style is to indent with four spaces.

`println!` is a macro... why? We'll find out in Chapter 19, it seems.

Rust ends lines with semicolons.

Rust compiles code, like how Java / Scala compile to JVM bytecode, or how C / C++ compile to binary executables.

> _"Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code. Next, we’ll introduce you to the Cargo tool, which will help you write real-world Rust programs."_

## Hello, Cargo!

 "Cargo is Rust’s build system and package manager."

 - builds code
 - downloads libraries ("dependencies", "crates")
 - builds libraries

Check if `cargo` is installed with `cargo --version`

`cargo new <project_name>` to create a new Cargo project

Cargo projects are configured using TOML https://toml.io/

Rust _editions_
 - https://doc.rust-lang.org/book/appendix-05-editions.html
 - https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field
 - https://doc.rust-lang.org/edition-guide/editions/index.html
 - editions do not split the ecosystem
 - similar to language versions, but opt-in, and configurable per-crate
 - "...the decision to migrate to a newer edition is a 'private one' that the crate can make without affecting others."
 - automatically update to a new edition via `cargo fix`

Source code goes in a `/src` directory in the project root

`Cargo.toml` goes in the project root directory `/`

Build a Cargo project with `cargo build`
 - builds to `/target/debug` instead of current directory `/`
 - (default build is a debug build)

`Cargo.lock` lockfile for dependency versions

Build and run with `cargo run`

Just check compilation with `cargo check` -- doesn't produce an executable (faster)
 - _How can I get cargo to recompile changed files automatically?_ -- `cargo-watch` https://stackoverflow.com/a/38117745/2925434

 Use `cargo build --release` to compile with optimizations and create an artifact
  - creates an executable in `/target/release` instead of `/target/debug`
  - takes longer to compile, but executable will run faster