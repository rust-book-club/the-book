# Chapter 8: Common Collections

Rust has built-in: primitives, arrays, tuples, structs.

_Collections_ of objects are stored on the heap.

This chapter discusses _vectors_, _strings_, and _hash maps_.

The [standard library](https://doc.rust-lang.org/std/collections/index.html) also provides
- `VecDeque`, `LinkedList`, `BTreeMap`, `HashSet`, `BTreeSet`, and `BinaryHeap` objects

## Storing Lists of Values with Vectors

A vector "puts all the values next to each other in memory" -- I guess this means when we resize, we need to reallocate a new chunk of memory on the heap big enough to hold everything.

### Creating a New Vector

Create a new vector with

```rust
let v: Vec<i32> = Vec::new(); // or
let v = vec![1, 2, 3]; // macro
```

### Updating a Vector

Add elements with `push()`

```rust
let mut v = Vec::new();
v.push(5);
```

### Reading Elements of Vectors

Vector elements can be accessed by indexing, or with get

```rust
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2]; // third is of type &i32
let third: Option<&i32> = v.get(2); // third is an Option
```

Direct access will `panic` if the index is out of range.

A mutable reference to _any element_ of a vector, or the vector itself, is incompatible with having any other references to any other elements. Why?

> _"...adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored."_

### Iterating over the Values in a Vector

Prefer a `for` loop to directly iterating over vector indices

```rust
let v = vec![100, 32, 57];
for n_ref in &v {
    // n_ref has type &i32
    let n_plus_one: i32 = *n_ref + 1; // dereference n_ref to add to it
    println!("{}", n_plus_one);
}
```

We can also iterate over mutable references to each element

```rust
let mut v = vec![100, 32, 57];
for n_ref in &mut v {
    // n_ref has type &mut i32
    *n_ref += 50;
}
```

It's safe to loop over a vector this way. The borrow checker ensures that we will not increase or decrease the size of the vector while we are looping over it.

### Using an Enum to Store Multiple Types

Pro tip: store heterogeneous data in a vector by using an enum

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### Dropping a Vector Drops Its Elements

```rust
{
    let v = vec![1, 2, 3, 4];
} // <- v goes out of scope and is freed here, along with all its elements
```

## Storing UTF-8 Encoded Text with Strings

### Creating a New String

A `String` is just "a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities". It's a fancy `Vec<T>`.

Rust's `to_string` ("available on any type that implements the `Display` trait") is similar to Java's / Scala's `toString`

```rust
let mut s = String::new();                // String
let data = "initial contents";            // &str
let s = data.to_string();                 // String
let s = "initial contents".to_string();   // String
let s = String::from("initial contents"); // String
```

Strings are UTF-8 encoded.

### Updating a String

#### Appending to a String with `push_str` and `push`

Append a string slice to a `String` with `push_str`

```rust
let mut s = String::from("foo");
s.push_str("bar"); // foobar
```

Append a single character with `push`

```rust
let mut s = String::from("lo");
s.push('l');
```

#### Concatenation with the `+` Operator or the `format!` Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The `+` operator uses the `add` method which looks kind of like this

```rust
fn add(self, s: &str) -> String {
//     ^        ^ doesn't take ownership of s2
//     | _does_ take ownership of s1
```

This method is actually generic (Chapter 10), but the book has replaced the generic types with the appropriate string types here. Also `&s2` is coerced from a `&String` to a `&str` with deref coercion (Chapter 15).

When adding multiple strings together, `format!` is often more readable

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;      // don't do this
let s = format!("{}-{}-{}", s1, s2, s3); // do this instead
```

The `+` operator takes ownership of the first parameter, but the `format!` macro doesn't take ownership of any of its arguments.

### Indexing into Strings

Rust strings cannot be indexed by integers

```rust
let s1 = String::from("hello");
let h = s1[0]; // the trait `Index<{integer}>` is not implemented for `String`
```

...because UTF-8 characters are multi-byte.

### Bytes and Scalar Values and Grapheme Clusters! Oh My!

The string "नमस्ते" is composed of the bytes

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

or, equivalently, of the Rust `char`s (Unicode scalar values)

```
['न', 'म', 'स', '्', 'त', 'े']
```

or, again eqiuvalently, of the grapheme clusters

```
["न", "म", "स्", "ते"]
```

...so Rust provides different ways of accessing this kind of data, rather than assuming what the user wants when they say `s1[0]`.

### Slicing Strings

A string slice is a range of bytes

```rust
let hello = "Здравствуйте";
let s = &hello[0..4]; // Зд
```

Slicing a string in the middle of a `char` can lead to runtime panics like

```rust
&hello[0..1]; // byte index 1 is not a char boundary; it is inside 'З'
```

So be very careful when creating string slices on arbitrary string data.

### Methods for Iterating Over Strings

Rust provides a `.chars()` method and a `.bytes()` method for iterating over the individual `char`s and bytes that make up a string

```rust
"Зд".chars() // ['З', 'д']
"Зд".bytes() // [208, 151, 208, 180]
```

There is no built-in functionality for iterating over grapheme clusters, because it's really complex. There might be some libraries on `crates.io` to do this though. (Like [this one](https://crates.io/crates/unicode-segmentation).)

### Strings Are Not So Simple

> _"Rust has chosen to make the correct handling of `String` data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle."_

## Storing Keys with Associated Values in Hash Maps

### Creating a New Hash Map

```rust
fn main() {
    use std::collections::HashMap; // not automatically brought into scope

    let mut scores = HashMap::new(); // no built-in macro like map! to construct them

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

There's a pretty full-featured [`map!` (also `set!`) macro crate here](https://crates.io/crates/map-macro).

But "[m]any collections now offer conversions from an array argument using From or Into" [(source)](https://stackoverflow.com/a/27582993/2925434)

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn main() {
    let s = Vec::from([1, 2, 3]);
    let s = BTreeSet::from([1, 2, 3]);
    let s = HashSet::from([1, 2, 3]);
    let s = BTreeMap::from([(1, 2), (3, 4)]);
    let s = HashMap::from([(1, 2), (3, 4)]);
}
```

> _"Just like vectors, hash maps store their data on the heap. This `HashMap` has keys of type `String` and values of type `i32`. Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type."_

### Accessing Values in a Hash Map

The Book doesn't explain this here, [but any type that implements `Index` can be accessed with `this[syntax]`](https://stackoverflow.com/a/69342646/2925434), so a `HashMap` can be accessed directly like `map["key"]`, just like a Vector with its integer index.

The Book only discusses accessing map values using `get()` or a `for` loop

```rust
let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

These two methods (`[]` and `get`) are similar to the two ways to access `Map` values in Scala (`()` and `get`). The former causes a panic / throws an exception if the key doesn't exist, while the latter returns an optional value which is none if the key doesn't exist.

Note that, in some cases, [`panic`s can be caught in Rust](https://stackoverflow.com/a/73224634/2925434), just like thrown exceptions can be caught in JVM languages

```rust
fn main() {
    let v = vec![1, 2, 3];
    let panics = std::panic::catch_unwind(|| v[99]).is_err();
    assert!(panics);
    println!("Hello, World");
}
```

### Hash Maps and Ownership

Heap values `insert`ed into a hash map are moved into the map, which takes ownership of them.

A hash map doesn't take ownership of references `insert`ed, but the lifetime of those references must be valid for at least as long as the hash map is alive. See Chapter 10.

### Updating a Hash Map

#### Overwriting a Value

`insert` overwrites the value in the map with the given key if it already exists

#### Adding a Key and Value Only If a Key Isn't Present

Use `entry` and `or_insert` to write a value only if the key doesn't already exist

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);            // Blue -> 10
scores.entry(String::from("Yellow")).or_insert(50); // Yellow -> 50
scores.entry(String::from("Blue")).or_insert(50);   // Blue -> 10 still
```

#### Updating a Value Based on the Old Value

`or_insert` returns a mutable reference to the `Entry`, so we can use it to update the value if it exists

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); // read existing count, or default to 0
    *count += 1; // increment word count
}
```

### Hashing Functions

You can provide a custom hashing function to `HashMap`, and many crates are available which provide hashing functions different from the default _SipHash_ method.

