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

### Making Structs and Enums Public

> _"If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis."_

> _"In contrast, if we make an enum public, all of its variants are then public. We only need the `pub` before the `enum` keyword..."_

## Bringing Paths into Scope with the `use` Keyword

Like inline `import`s in Scala

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // fails, not in scope
    }
}
```

### Creating Idiomatic `use` Paths

Prefer

```rust
use crate::front_of_house::hosting;
hosting::add_to_waitlist();
```

to

```rust
use crate::front_of_house::hosting::add_to_waitlist;
add_to_waitlist();
```

Just like in Scala

```scala
import com.mypackage.MyActor
target ! MyActor.MyMessage
```

rather than

```scala
import com.mypackage.MyActor.MyMessage
target ! MyMessage
```

Conflicting names can be resolved by only bringing the parent modules into scope

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

### Providing New Names with the as Keyword

...or by using the `as` keyword, like in Scala

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with `pub use`

```rust
pub use crate::front_of_house::hosting;
```

brings `hosting` into scope and also exports it from this module, so other modules bringing this one into scope can refer to `hosting` as well.

### Using Nested Paths to Clean Up Large `use` Lists

You can collapse `use` statements in Rust just like `import` statements in Scala

```rust
use std;
use std::cmp::Ordering;
use std::io;
```

is equivalent to

```rust
use std::{self, cmp::Ordering, io};
```

### The Glob Operator

```rust
use std::collections::*;
```

brings all public items from `collections` into scope.

## Separating Modules into Different Files

> _"...you only need to load a file using a `mod` declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the `mod` statement), other files in your project should refer to the loaded file's code using a path to where it was declared... In other words, **`mod` is not an 'include' operation that you may have seen in other programming languages.**_"

`mod` vs `use`

> *"The compiler knows to look in this file because it came across the module declaration __in the crate root__ with the name `front_of_house`."*

### Summary

> _"Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the `pub` keyword."_