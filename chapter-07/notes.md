# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

> *"A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. ... For very large projects comprising a set of interrelated packages that evolve together, Cargo provides* workspaces, *which we’ll cover in the [“Cargo Workspaces”](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) section in Chapter 14."*

## Packages and Crates

> _"A_ crate _is the smallest amount of code that the Rust compiler considers at a time."_

> _"Crates can contain modules, and the modules may be defined in other files that get compiled with the crate..."_

_binary_ crates and _library_ crates

Binary crates
- compile to an executable
- must have a function called `main`

Library crates
- don't compile to an executable
- don't need to have a `main` function

> _Most of the time when Rustaceans say "crate", they mean library crate, and they use "crate" interchangeably with the general programming concept of a "library"._

> _"The_ crate root _is a source file that the Rust compiler starts from and makes up the root module of your crate..."_

> _"A_ package _is a bundle of one or more crates that provides a set of functionality. A package contains a_ Cargo.toml _file that describes how to build those crates."_

> _"A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate."_

The crate root is at
- `src/main.rs` for binary crates
- `src/lib.rs` for library crates

> _"A package can have multiple binary crates by placing files in the_ src/bin _directory: each file will be a separate binary crate."_

## Defining Modules to Control Scope and Privacy

_paths_ name items

`use` brings a _path_ into scope

### Modules Cheat Sheet

Declare a new module named (for example) `garden` in the crate root with `mod garden;`. Rust will look for the module's code in
- `{}` after the word `garden`, in place of the `;`
- `src/garden.rs`
- `src/garden/mod.rs`

Submodules are similar, and can be declared in any file other than the crate root. (ex. `mod vegetables { ... }` in `garden.rs`)

As long as privacy rules allow, you can then refer to types in any submodule by their path, e.g. `crate::garden::vegetables::Asparagus`

Module internals are private by default. Public modules are declared with `pub mod`. Items in a module must also explicitly be declared `pub`lic.

`use` is like `import` in Scala

```rust
use crate::garden::vegetables::Asparagus;
Asparagus::doSomething();
```

```scala
import crate.garden.vegetables.Asparagus
Asparagus.doSomething()
```

### Grouping Related Code in Modules

> _"...code within a module is private by default..."_

## Paths for Referring to an Item in the Module Tree

Paths can be _absolute_ or _relative_.

Relative paths start with `self`, `super`, or an identifier in the current module.

### Exposing Paths with the `pub` Keyword

#### Best Practices for Packages with a Binary and a Library

> _"The module tree should be defined in_ src/lib.rs."

### Starting Relative Paths with `super`

Like `..` in a filesystem.