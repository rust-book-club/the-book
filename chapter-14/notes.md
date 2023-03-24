# Chapter 14: More About Cargo and Crates.io

This chapter will explain how to

- Customize your build through release profiles
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands

## Customizing Builds with Release Profiles

`dev` and `release` are the two main release profiles defined by Cargo

```sh
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

"release profiles are predefined and customizable"

"allow a programmer to have more control over various options for compiling code"

Add `[profile.*]` sections to _Cargo.toml_ to customize or override release profile settings

"For example, here are the default values for the `opt-level` setting for the `dev` and `release` profiles:"

```
// Cargo.toml
[profile.dev]
opt-level = 0 // number of optimizations -> 0 to 3

[profile.release]
opt-level = 3 // more optimizations = longer compile time, but better runtime performance
```

[Here is the documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) around release profiles.

## Publishing a Crate to Crates.io

### Making Useful Documentation Comments

Rust doc(umentation) comments start with `///` and use Markdown notation. For example

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Generate HTML documentation from doc comments with `cargo doc --open`. This runs `rustdoc`, puts the generated HTML in the _target/doc_ directory, and opens the documentation in the browser.

#### Commonly Used Sections

- `# Examples`
- `# Panics`: The scenarios in which the function being documented could panic
- `# Errors`: If the function returns a `Result`, describes the kinds of errors that might occur and what conditions might cause those errors to be returned
- `# Safety`: If the function is `unsafe` to call, there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold

#### Documentation Comments as Tests

`cargo test` will run the code example above as a `Doc-test`:

```
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

This keeps documentation in sync with the code.

#### Commenting Contained Items

Use `//!` doc comments to document the item that contains the comments rather than the item directly beneath the comments. "We typically use these doc comments inside the crate root file (_src/lib.rs_ by convention) or inside a module to document the crate or the module as a whole." For example

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

### Exporting a Convenient Public API with `pub use`

> _"you can re-export items to make a public structure that’s different from your private structure by using `pub use`. Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead."_

This

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

turns this

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

into this

```rust
use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}
```

and also adds a _Re-exports_ section to the generated documentation.

### Setting Up a Crates.io Account

Requires a GitHub. Generate an API token and keep it secret.

### Adding Metadata to a New Crate

To the `[package]` section of the _Cargo.toml_ file.

Crates must have globally unique names across crates.io.

Also required: a description, a license.

> _"In_ Cargo.toml, _add a description that's just a sentence or two, because it will appear with your crate in search results. For the `license` field, you need to give a_ license identifier value. _The [Linux Foundation’s Software Package Data Exchange (SPDX)](http://spdx.org/licenses/) lists the identifiers you can use for this value."_

Example:

```rust
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

### Publishing to Crates.io

Crates.io is meant to act as a permanent, immutable archive of all versions of all Rust crates ever published. So publishing a version of a crate is permanent, and cannot be altered or undone.

Publish with `cargo publish`.

### Publishing a New Version of an Existing Crate

Change the `version` in Cargo.toml according to semantic versioning rules and `cargo publish` again.

### Deprecating Versions from Crates.io with `cargo yank`

> _"Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue. Essentially, a yank means that all projects with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version."_

```sh
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game:1.0.1
```

You can undo a `yank` with

```sh
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game_:1.0.1
```

Literally incredible: "if you want Rust to treat undocumented code as an error, you can add the following statement at the root of your library:"

```rust
#![deny(rustdoc::missing_docs)]
```

## Cargo Workspaces

### Creating a Workspace

> _"A_ workspace _is a set of packages that share the same_ Cargo.lock _and output directory."_

A workspace allows for multiple library crates.

```sh
mkdir add
cd add
```

```toml
# add/Cargo.toml
[workspace]

members = [
  "adder",
]
```

```sh
cargo new adder
cargo build
```

```
add
├── Cargo.lock
├── Cargo.toml
├── adder           <- adder package doesn't have its own target directory
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target          <- compiled artifacts will be placed here
```

### Creating the Second Package in the Workspace

```toml
# add/Cargo.toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

```sh
cargo new add_one --lib
```

```
add
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

```rust
// add/add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

```toml
# add/adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }
```

```rust
// add/adder/src/main.rs
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

```sh
cargo build
cargo run # prints: Hello, world! 10 plus one is 11!
```

#### Depending on an External Package in a Workspace

There is only one _Cargo.lock_ file, at the top level, to ensure that all crates in the workspace will always be compatible with one another.

But I seem to be able to specify `0.8.3` in one crate and `0.7.0` in another, and both get added to the root `Cargo.lock`... what gives?

> _"Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock. Making all crates in the workspace use the same dependencies means the crates will always be compatible with each other."_ -- unless those two versions are incompatible?

Seems to be the case. With `0.8.3` and `0.8.5`, both are resolved to `0.8.5`... Seems like a headache to keep track of.

(This is explained in the quiz at the end of this section of the interactive book.)

#### Adding a Test to a Workspace

`cargo test` runs all tests in all crates

`cargo test -p my_package` runs only the tests in `my_package`

Crates in a workspace must be published individually to crates.io, as well, using `-p`

```sh
cargo publish -p my_package
```

## Installing Binaries with `cargo install`

"The `cargo install` command allows you to install and use binary crates locally."

```sh
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
--snip--
   Compiling ripgrep v11.0.2
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
```

## Extending Cargo with Custom Commands

> _"If a binary in your `$PATH` is named `cargo-something`, you can run it as if it was a Cargo subcommand by running `cargo something`. Custom commands like this are also listed when you run `cargo --list`."_

