# Chapter 18: Patterns and Matching

Rust can _destructure_ arrays, enums, structs, and tuples.

"the difference between refutable and irrefutable patterns"

## All the Places Patterns Can Be Used

### `match` Arms

`match` arms must be _exhaustive_

```rs
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

The pattern `_` matches anything and doesn't bind to a variable, so it is often used as the last "catch-all" arm of a `match` expression.

### Conditional `if let` Expressions

Can be used "as a shorter way to write the equivalent of a `match` that only matches one case".

`if let` can have a corresponding `else`

```rs
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");

    // else if
    } else if is_tuesday {
        println!("Tuesday is green day!");

    // else if let
    } else if let Ok(age) = age { // age is shadowed here

        // ...

    // else
    } else {
        println!("Using blue as the background color");
    }
}
```

### `while let` Conditional Loops

"the `while let` conditional loop allows a `while` loop to run for as long as a pattern continues to match"

```rs
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
```

### `for` Loops

```rs
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
```

### `let` Statements

```rs
    let x = 5; // simple variable assignment
    let (x, y, z) = (1, 2, 3);
    let (x, y) = (1, 2, 3); // will not compile
```

### Function Parameters

```rs
// a simple function
fn foo(x: i32) {
    // code goes here
}
```

```rs
// destructure the tuple
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

## Refutability: Whether a Pattern Might Fail to Match

"Patterns that will match for any possible value passed are _irrefutable_."

ex. `let x = 5;`

"Patterns that can fail to match for some possible value are _refutable_."

ex. `if let Some(x) = a_value;`

> "Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match. The `if let` and `while let` expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure."

The compiler gives a warning if an irrefutable pattern is used where a refutable pattern is expected, for example

```rs
if let x = 5 {
  // this pattern will never fail
}
```

## Pattern Syntax

### Matching Literals

```rs
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

### Matching Named Variables

```rs
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // set y to 5 here
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
```

### Multiple Patterns

```rs
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // | operator here
        3 => println!("three"),
        _ => println!("anything"),
    }
```

### Matching Ranges of Values with `..=`

```rs
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
```

### Destructuring to Break Apart Values

#### Destructuring Structs

```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p1; // destructure p1 here
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p2 = Point { x: 0, y: 7 };

    let Point { x, y } = p2; // assign p2.x to x, p2.y to y
    assert_eq!(0, x);
    assert_eq!(7, y);

    // destructure in `match` expression arms
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

#### Destructuring Enums

All of this is very similar to Scala

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}
```

#### Destructuring Nested Structs and Enums

```rs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color), // an enum variant containing an enum variant
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}
```

#### Destructuring Structs and Tuples

```rs
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### Ignoring Values in a Pattern

#### Ignoring an Entire Value with `_`

This will match any value but not bind to the value

```rs
fn foo(_: i32, y: i32) { // we ignore the first parameter here
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

#### Ignoring Parts of a Value with a Nested `_`

```rs
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // we ignore the values contained within the Somes
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
```

Another example

```rs
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
```

#### Ignoring an Unused Variable by Starting Its Name with `_`

```rs
fn main() {
    let _x = 5; // unused, but will not generate a warning
    let y = 10; //                 will generate a warning
}
```

"Note that there is a subtle difference between using only `_` and using a name that starts with an underscore. The syntax `_x` still binds the value to the variable, whereas `_` doesn’t bind at all."

This means the following example won't compile, because the `s` value is moved into `_s`

```rs
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
```

#### Ignoring Remaining Parts of a Value with `..`

Use `..` to ignore all remaining parts of a value with many parts, rather than a `_` for each part.

```rs
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
```

...and with a tuple...

```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

Using `..` must be unambiguous, of course. The following example does not compile

```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

It is not valid to match on any array or tuple with two `..`. (Is this true?)

### Extra Conditionals with Match Guards

Just like in Scala

"...an additional `if` condition, specified after the pattern in a `match` arm..."

```rs
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x), // "if x % 2 == 0" is a match guard
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
```

"...the compiler doesn't try to check for exhaustiveness when match guard expressions are involved."

"You can also use the or operator `|` in a match guard to specify multiple patterns; the match guard condition will apply to all the patterns." For example

```rs
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // "if y" applies to 4, 5, and 6
        _ => println!("no"),
    }
```

### `@` Bindings

Just like in Scala

"The _at_ operator `@` lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match."

```rs
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // "..=" pattern match here, but also assignment to id_variable
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
```

## Quiz

I was very surprised that this worked

```rs
let a = [(0, 1)];
let ?? = a;
```

when `??` was `[(n, ..)]`, because I thought the type of `a` would be `[(i32, i32)]` (of unknown length), but it's actually `[(i32, i32); 1]` -- The compiler _knows the length of the array_
