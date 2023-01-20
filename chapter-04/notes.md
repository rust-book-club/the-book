# Chapter 4: Understanding Ownership

## What is Ownership?

> "Ownership _is a set of rules that govern how a Rust program manages memory."_

In opposition to: garbage collection, explicit memory allocation and deallocation.

### The Stack and the Heap

Data with a known, fixed size can be aded to the stack, in LIFO order.

Data with an unknown or variable size must be added to the heap.

_Allocating_ on the heap consists of
- finding a chunk of memory big enough for your request
- receiving a pointer to that memory location

Pointers are a known, fixed size and so can be stored on the stack.

Pushing data to the stack is faster because the allocator never needs to look for free space, it just adds the data to the top of the stack.

> _"When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack."_

Ownership...
- keeps track of what parts of your code are using data on the heap
- minimizes the amount of duplicate data on the heap
- cleans up unused data on the heap so you don't run out of space

Related: [Are there any alternatives to the stack+heap+static memory model?](https://softwareengineering.stackexchange.com/q/222564/294935) (A: No, not really. Like RAM vs. hard-disk storage, or different levels of processor caches, the division just sort of "makes sense".)

### Ownership Rules

1. Each value in Rust has an _owner_.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### The `String` Type

Ownership does not apply to values on the stack, like the scalar types integers, floating-point numbers, Booleans, and characters.

String literals, hard-coded into the program, are also stored on the stack, because their size is known at compile time. To turn a string literal into a `String`, use `String::from()`:

```rust
let s = String::from("hello");
```

`String`s can be mutated, but string literals cannot. Ex:

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```

### Memory and Allocation

We request memory from the allocator at runtime when we call `String::from()`. But we must also return this memory to the allocator when we're done with our `String`. In GC languages, this is done automatically via reference counting or similar techniques.

> _"Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope..._
> 
> ```rust
> {
>     let s = String::from("hello"); // s is valid from this point forward
>     // do stuff with s
> }   // this scope is now over, and s is no longer valid
> ```

Rust calls [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop) to return this value's memory to the allocator. You can override `drop` to define custom destructor behaviour, like closing a socket or a file, etc.

#### Variables and Data Interacting with Move

Copying ("shallow copying" in other languages) a heap-allocated value like

```rust
let s1 = String::from("hello");
let s2 = s1;
```

...means that `s1` goes out of scope when `s2` is created. In other languages, we might get two references to the same data on the heap, but then we need to worry about "double free" errors when we clear the same data from the heap twice. Rust side steps all of this.

This is called "moving" in Rust, because in addition to making a shallow copy, Rust invalidates the first reference.

#### Variables and Data Interacting with Clone

Duplicate data on the heap with `clone`, which does a "deep copy" of a value:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```

#### Stack-Only Data: Copy

Types which implement the `Copy` trait exist on the stack and are cloned automatically

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```

`Copy` is implemented by the four scalar types (integers, floating-point numbers, Booleans, and characters), as well as static strings, and any combination of these types in a tuple, as well as any array of any of these types.

Types which exist on the heap and must be destructed (like `String`) implement the `Drop` trait, so that their `drop` implementation can be called when they go out of scope.

A type cannot implement both `Drop` and `Copy`.

### Ownership and Functions

> _"The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does."_

```rust
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
```

### Return Values and Scope

It seems like the way to think about this is that variables "move into" functions, and if a value isn't returned, it's consumed.

> _"While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership?"_

...that's where _borrowing_ comes in!

## References and Borrowing

> _"A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference."_

(A reference is dereferenced with `*`, the dereference operator, which we'll see in Chapters 8 and 15.)

Without references

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
```

with references (without `calculate_length` taking ownership of the `String` passed to it)

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, that value is not dropped.
```

The type `&String` represents a reference to a `String`, rather than the `String` itself.

> _"The `&s1` syntax lets us create a reference that refers to the value of `s1` but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used."_

> _"When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership."_

### Mutable References

> _"Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to."_

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

__Question:__ Are the below cases exhaustive?

So there are two concepts here: borrowing and mutability. You can have
1. an immutable object that you take ownership of
2. an immutable object that you borrow
3. a mutable object that you take ownership of
4. a mutable object that you borrow, but are not allowed to mutate *
5. a mutable object that you borrow, and are allowed to mutate *

\* You can only have one mutable reference to a value at any given time. This prevents data races.

\* You also cannot create a mutable reference to a value while there are immutable references to that same value.

Basically, an immutable reference allows you to _read_ the data, while a mutable reference allows you to _read and write_ the data.

__Question:__ Can I have a mutable borrow of an immutable value?

### Dangling References

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
```

> _"The solution here is to return the `String` directly... This works without any problems. Ownership is moved out, and nothing is deallocated."_

### The Rules of References

- At any given time, you can have _either_ one mutable reference _or_ any number of immutable references.
- References must always be valid.

## The Slice Type

> "Slices _let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership."_

```rust
let s = String::from("hello world");
let hello = &s[0..5]; // type is &str
let world = &s[6..11];
```

> _"String slice range indices must occur at valid UTF-8 character boundaries."_

#### String Literals as Slices

> _"Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:_
> 
> ```rust
> let s = "Hello, world!";
> ```
> 
> _The type of `s` here is `&str`: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an immutable reference."_

#### String Slices as Parameters

Prefer `&str` to `&String`: `&String` can be coerced to `&str` (Implicit Deref Coercions, Chapter 15) but not the other way around.

### Other Slices

We can also take slices of arrays

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // type is &[i32]
```