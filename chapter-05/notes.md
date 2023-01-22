# Chapter 5: Using Structs to Structure Related Data

Structs have "associated functions" (_"methods"_).

> _"Structs and enums... are the building blocks for creating new types in your program’s domain..._"

## Defining and Instantiating Structs

`struct`s in Rust are like `struct`s in C, they can have any number of fields of any type, and the fields are named.

```rust
// Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
};
```

```c
// C
struct MyStructure {
  int myNum;
  char myLetter;
};
```

Structs are like tuples, except each element has a name, and the collection of elements has a user-defined type and name. This makes them like a usually-easier-to-use version of a tuple.

To create an _instance_ of a struct in Rust, we use syntax like

```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

> _"We don’t have to specify the fields in the same order in which we declared them in the struct."_

Just like in C, _dot notation_ is used to access values in a struct

```rust
user1.email = String::from("anotheremail@example.com");
```

> _"Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable."_

### Using the Field Init Shorthand

Similar to [JavaScript ES6](https://ui.dev/shorthand-properties)

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

is equivalent to

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

because of the shorthand field initializer syntax.

### Creating Instances From Other Instances With Struct Update Syntax

Keep most of the values of a struct, updating only a few, with the _struct update syntax_

```rust
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

is equivalent to

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

Note
- `user1.username` is _moved_ into `user2`, and so
- `user1` is no longer valid after creating `user2`
- if we had given `user2` a new `username`, this wouldn't be the case

__Note:__ [_"Rust doesn't have classes"_](https://www.electronicdesign.com/blogs/altembedded/article/21252370/electronic-design-an-objective-look-at-rust), instead it has `trait`s which are similar to Java's old `Interface`s, [before they could have default implementations](https://www.veracode.com/blog/secure-development/java-8-default-interface-methods).

### Using Tuple Structs without Named Fields to Create Different Types

_Tuple structs_ are basically just named tuples (or, similar to "tagged types" in Scala)

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

They allow for better semantic safety. Their fields are unnamed. So Rust uses _nominal typing_ rather than _structural typing_ like in languages like JavaScript and TypeScript.

### Unit-Like Structs Without Any Fields

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

> _"Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself."_

Need to learn more about _lifetimes_...

These `User`s use `String`s and not `&str`s because the latter are references to data that "someone else" owns. We need to introduce a lifetime parameter if this struct is going to have a reference to some external data, so that "the data referenced by [the] struct is valid for as long as the struct is".

## An Example Program Using Structs

### Refactoring with Structs: Adding More Meaning

> _"...note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs..."_

### Adding Useful Functionality with Derived Traits

`#[derive(Debug)]` allowed Rust to automatically derive a `Debug` trait implementation for our `Rectangle` struct. Other derivable traits listed in [Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) include

`derive` can be applied to a `struct` or an `enum` definition (not tuples, though, I guess).

> _"The derive attribute generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax."_

Every trait in the standard library that can be used with `derive`:
- `Debug` for programmer output
   - required for `assert_eq!` macro
- `PartialEq` for equality comparisons
   - enables use of the `==` and `!=` operators
   - implements the `eq` method
   - also required for `assert_eq!` macro
- `Eq` for self-equality
   - not implemented by e.g. `NaN`
   - `PartialEq` required for `Eq`
- `PartialOrd` for ordering comparisons
   - provides the `<`, `>`, `<=`, `>=` operators
   - `PartialEq` required for `PartialOrd`
   - `partial_cmp` produces an `Option<Ordering>`
      - e.g. a float and `NaN` produces `None`
   - required for e.g. `gen_range` method of `rand` crate
- `Ord` for guaranteed ordering
   - `cmp` produces an `Ordering`; valid ordering always possible
   - `PartialOrd` and `Eq` required for `Ord`
- `Clone` and `Copy` for duplicating values
   - `Clone` deep-copies data stored on the heap
   - `Copy` duplicates data stored on the stack
   - `Clone` is required for `Copy`
- `Hash` for mapping a value to a value of fixed size
   - provides the `hash` method
- `Default` for default values
   - provides a `default` method
   - required for `unwrap_or_default` method on `Option<T>` values

> _"The `Default::default` function is commonly used in combination with the struct update syntax discussed in... Chapter 5. You can customize a few fields of a struct and then set and use a default value for the rest of the fields by using `..Default::default()`."_

The [standard library documentation](https://doc.rust-lang.org/std/index.html) gives details on how to manually implement these traits, if the default `derive`d implementations are not what you want.

> _"These traits listed here are the only ones defined by the standard library that can be implemented on your types using `derive`. Other traits defined in the standard library don’t have sensible default behavior, so it’s up to you to implement them in the way that makes sense for what you’re trying to accomplish."_

Libraries can implement `derive` for their own traits, using a procedural macro, covered in the ["Macros" section of Chapter 19](https://doc.rust-lang.org/book/ch19-06-macros.html#macros).

## Method Syntax

"Methods" are functions which
- are defined within the context of a struct, enum, or trait
- have `self` as the first parameter

### Defining Methods

