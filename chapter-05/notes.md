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

