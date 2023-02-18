# Chapter 11: Writing Automated Tests

## How to Write Tests

### The Anatomy of a Test Function

> _"At its simplest, a test in Rust is a function that’s annotated with the `test` attribute."_

We used the `derive` attribute in Chapter 5.

To write and run a test
- add `#[test]` on the line before the `fn`
- call `cargo test`

When creating a new library project with Cargo, an example test is added automatically

```rust
cargo new adder --lib
```

```rust
// adder/src/lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // the test runner will treat this function as a test
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

> _"Rust can compile any code examples that appear in our API documentation. This feature helps keep your docs and your code in sync! We’ll discuss how to write documentation tests in the [“Documentation Comments as Tests”](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests) section of Chapter 14."_

> _"Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed."_

Use
- `assert!`, `assert_eq!`, `assert_ne!`, etc.

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

> _"Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. For structs and enums that you define yourself... this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition."_

### Adding Custom Failure Messages

> _"Any arguments specified after the required arguments are passed along to the `format!` macro..."_

So while

```rust
assert_eq!(4, add_two(3));
```

results in

```rust
  left: `4`,
 right: `5`', src/lib.rs:14:9
```

```rust
assert_eq!(4, add_two(3), "{} should equal {}", 4, add_two(3));
```

results in

```rust
  left: `4`,
 right: `5`: 4 should equal 5', src/lib.rs:14:9
```

### Checking for Panics with `should_panic`

> _"We do this by adding the attribute `should_panic` to our test function."_

