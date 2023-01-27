# Chapter 6: Enums and Pattern Matching

## Defining an Enum

Note: there are `unimplemented!()` and `todo!()` macros in the Rust `std` library.

Use [pattern matching](https://stackoverflow.com/q/9109872/2925434) to access `enum` values in Rust.

### Enum Values

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

is a much less verbose way to write

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

..._plus_ it groups these four kinds of messages under one type.

_Plus plus_ you can define methods on enums

```rust
impl Message {
    fn call(&self) {
        todo!("method body should be defined here")
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The `Option` Enum and Its Advantages Over `Null` Values

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`T` is a type parameter, and `Option<T>` is a parameterized ("generic") type.

## The `match` Control Flow Construct

> _"Chapter 18 covers all the different kinds of patterns and what they do."_

`match`es must be exhaustive.

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // if using {}, a comma is optional
        Coin::Nickel => 5, // these are called "arms"
        Coin::Quarter => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 10, // Coin::Dime
    }
}
```

## Concise Control Flow with `if let`

`if let` lets you handle values that match one pattern while ignoring the rest

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

is equivalent to

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

> _"...you can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values."_

You can use an `else` with an `if let`, as well

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

is equivalent to

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

Kind of a niche situation, but good to keep in mind.