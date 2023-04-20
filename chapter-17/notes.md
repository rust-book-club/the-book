# Chapter 17: Object-Oriented Programming Features of Rust

> "In this chapter, we’ll explore certain characteristics that are commonly considered object-oriented and how those characteristics translate to idiomatic Rust."

## Characteristics of Object-Oriented Languages

Arguably, OOP languages share certain common characteristics, namely
- objects,
- encapsulation, and
- inheritance.

### Objects Contain Data and Behavior

> "Object-oriented programs are made up of objects. An _object_ packages both data and the procedures that operate on that data. The procedures are typically called _methods_ or _operations_."

> "Using this definition, Rust is object-oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums. Even though structs and enums with methods aren’t _called_ objects, they provide the same functionality..."

### Encapsulation that Hides Implementation Details

> "If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement. The option to use pub or not for different parts of code enables encapsulation of implementation details."

### Inheritance as a Type System and as Code Sharing

> "If a language must have inheritance to be an object-oriented language, then Rust is not one. There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro."

In an object-oriented program, you might implement a method on a class, and use (or override) that implementation in a subclass. This can be done in Rust with traits with default implementations.

In an object-oriented program, you might want to substitute subclass B for class A in a method call. This is allowed because of class _inheritance_ / _polymorphism_.

"Inheritance has recently fallen out of favor" because
- "it's often at risk of sharing more code than necessary"
- "introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass"
- some languages only allow single inheritance, restricting program design

"Program to an interface" instead.

> "Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called _bounded parametric polymorphism_."

## Using Trait Objects That Allow for Values of Different Types

Common example of a GUI library with objects with a `draw()` method -- we can't possibly know every kind of object which might ever exist at compile time.

### Defining a Trait for Common Behavior

```rs
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // "`dyn Draw`" means "any type which implements `Draw`"
}                                       // `Box<dyn Draw>` is a _trait object_

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This solution is different from

```rs
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> // because here "T" can only be a _particular_ type
where             // like a Button or a TextBox, but not both of these
    T: Draw,      // This implementation would be _monomorphized_ at compile time
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Implementing the Trait

Example implementations

```rs
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

```rs
use gui::Draw;

struct SelectBox { // different fields from Button
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox { // different draw implementation
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

Declaring an instance of `Screen<T>` looks like

```rs
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

This is _duck typing_ -- it looks like a duck and quacks like a duck, so it must be a duck. In our case, it's `impl Draw` and has a `draw()` method, so it must be drawable. We don't care about any other internals of the type.

### Trait Objects Perform Dynamic Dispatch

Monomorphization of generic types means that Rust can do _static dispatch_, which is when the compiler knows which method you're calling at compile time.

Trait objects necessarily use _dynamic dispatch_, because the compiler cannot possibly know at compile time which types will be passed in as arguments.

There is a runtime cost associated with looking up the correct method to call, and dynamic dispatch prevents some other optimizations. So it should only be used when necessary.

## Implementing an Object-Oriented Design Pattern

This section discusses the _state pattern_ of object-oriented design. Sounds like actors moving through different behaviors. The example they give is of a blog post that (internally) can be in one of three states: draft, review, or published. External objects have no idea of the internal state of the blog post, but the post will behave differently based on what state it's in.

...

> "We need to set `state` to `None` temporarily rather than setting it directly with code like `self.state = self.state.request_review();` to get ownership of the `state` value. This ensures `Post` can’t use the old `state` value after we’ve transformed it into a new state."

### Trade-offs of the State Pattern

