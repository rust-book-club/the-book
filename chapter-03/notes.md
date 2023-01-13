# Chapter 3: Common Programming Concepts

Rust keywords and reserved words: https://doc.rust-lang.org/book/appendix-01-keywords.html

## Variables and Mutability

### Constants

Rust has mutable (`let mut`) and immutable (`let`) variables, but also constants (`const`). `const` values...
 - must have an annotated type
 - must be defined as a constant expression, known at compile time
 - should be written in `SCREAMING_SNAKE_CASE` by convention

There are actually quite a few expressions which are constant expressions (https://doc.rust-lang.org/reference/const_eval.html) including things like closures, `loop`s, and so on.

### Shadowing

> _"Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword."_

## Data Types

### Scalar Types

"Rust has four primary scalar types:
1. integers
2. floating-point numbers
3. Booleans, and
4. characters"

Integer types in Rust

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

`arch` integers will be 32-bit on 32-bit architecture, 64-bit on 64-bit architecture.

Integer literals can use the signed / unsigned type hints after the value, e.g. `57u8` is an unsigned 8-bit integer with the value `57`. Underscores `_` can also be used to make long numbers easier to read, e.g. `1_000_000` is equivalent to `1000000`.

There are also hexadecimal (e.g. `0xff`), octal (e.g. `0o77`), binary (e.g. `0b111_0000`), and byte (e.g. `b'A'`) literals. Use `isize` or `usize` when indexing some sort of collection.

##### Integer Overflow

When run in debug (build) mode, Rust will `panic` if integer overflow occurs. But when compiling in release mode with the `--release` flag, integer overflow will not cause a `panic`. Instead, wraparound will occur.

> _"To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:_
> - *Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.*
> - *Return the `None` value if there is overflow with the `checked_*` methods.*
> - *Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.*
> - *Saturate at the value’s minimum or maximum values with the `saturating_*` methods."*

#### Floating-Point Types

`f32` and `f64` are like Scala / Java's `Float` and `Double` types. `f64` is the default floating-point type.

#### Numeric Operations

_"Integer division truncates toward zero to the nearest integer."_

I read recently that a better way of doing this is to round toward the nearest even integer. Otherwise you can end up wth biases. Ex: (7.5, 8.5) rounds to (8, 8) so the average of the two numbers is still 8, rather than 8.5 (when 7.5 rounds to 8 and 8.5 rounds to 9).

#### The Character Type

> _"Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value."_

Rust charaters are surrounded by 's'ingle quotes and strings are surrounded by "double quotes".

### Compound Types

#### The Tuple Type

ex `let tup: (i32, f64, u8) = (500, 6.4, 1);`

Tuples can be destructured, like `let (x, y, z) = tup;`

...and their values can also be accessed individually (0-indexed)

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

Like in Scala, the empty tuple `()` signifies _unit_, which can be used as an empty return type.

#### The Array Type

Arrays in Rust are homogeneous and fixed-length, ex. `let a = [1, 2, 3, 4, 5];`

Arrays are allocated on the stack rather than the heap.

Vectors are a flexible-size, heap-allocated alternative to arrays.

Array types are written as `[<element type>; <length>]`, for example

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

> _"You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:_
> 
> ```rust
> let a = [3; 5];
> ```
> 
> _The array named `a` will contain `5` elements that will all be set to the value `3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise way._"

Array elements are accessed using square-brackets: `a[0]`, `a[1]`, etc.

Rust does array bounds-checking and `panic`s if there's an attempt to access an element beyond the end of an array, rather than simply allowing the user to access invalid memory.

## Functions

The `fn` keyword allows you to declare new functions.

Rust uses `snake_case` for function names by convention.

### Statements and Expressions

Rust makes the same distinction between "statements" and "expressions" that Scala does; in short, expressions evaluate to some value, while statements do not.

- variable assignment using `let` is a statement (can't do `let x = y = 6` in Rust)
- function definitions are statements
- mathematical operations are expressions (`5 + 6`)
- calling a function is an expression
- calling a macro is an expression

> _"A new scope block created with curly brackets is an expression, for example:_
> 
> ```rust
> fn main() {
>     let y = {
>         let x = 3;
>         x + 1
>     };
> 
>     println!("The value of y is: {y}");
> }
> ```
> 
> _This expression:_
> 
> ```rust
> {
>     let x = 3;
>     x + 1
> }
> ```
> 
> _is a block that, in this case, evaluates to `4`. That value gets bound to `y` as part of the `let` statement. Note that the `x + 1` line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value."_

__Be careful!__ Adding a semicolon to the end of a line turns that line into a statement, which does not return a value.

### Functions with Return Values

...are written using a `-> <return type>` like

```rust
fn five() -> i32 {
    5
}
```

Similar to Scala, the value of the last expression (in this case `5`) is used as the default return value from a function.

## Control Flow

### `if` Expressions

> _"Blocks of code associated with the conditions in `if` expressions are sometimes called_ arms, _just like the arms in `match` expressions..."_

#### Using `if` in a `let` Statement

`if`s are _expressions_, they return values, like in Scala, and their results can be assigned to variables.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

Rust will not do any kind of type coercion, as is done in Scala / Java, though. So

```rust
let condition = true;
let number = if condition { 5 } else { "six" };
```

...will throw an error, but so will

```rust
let condition = true;
let number = if condition { 5 } else { 6.0 };
```

...because `5` is an `i32` but `6.0` is an `f64`. Integers are not automatically widened to floating-point values in Rust. See: https://stackoverflow.com/a/39682485/2925434

### Repetition with Loops

#### Returning Values from Loops

When `break`ing a `loop`, you can place a return value after the `break` statement, like so

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {result}");
```

#### Loop Labels to Disambiguate Between Multiple Loops

Add a _loop label_ (which must start with an apostrophe `'`) to `break` an outer `loop` from within an inner one

```rust
'outer: loop {
  'inner: loop {
    loop {
      break 'outer;
    }
  }
}
```

#### Conditional Loops with `while`

Rust also provides `while` loops

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");
    number -= 1;
}

println!("LIFTOFF!!!");
}
```

#### Looping Through a Collection with `for`

Rust also provides `for` ... `in` style loops:

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```