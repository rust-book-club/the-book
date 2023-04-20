# Chapter 16: Fearless Concurrency

> "_Concurrent programming_, where different parts of a program execute independently, and _parallel programming_, where different parts of a program execute at the same time..."

> "For this chapter, please mentally substitute _concurrent and/or parallel_ whenever we use _concurrent_."

> "Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements."

This chapter covers

- threads
- message-passing concurrency with channels between threads
- shared-state concurrency between threads
- the `Sync` and `Send` traits

## Using Threads to Run Code Simultaneously

Operating system _processes_ can each have multiple _threads_.

> "The Rust standard library uses a _1:1_ model of thread implementation, whereby a program uses one operating system thread per one language thread."

### Creating a New Thread with `spawn`

`std::thread::spawn` takes a closure as an argument and runs that closure in a new thread.

> "Note that when the main thread of a Rust program completes, all spawned threads are shut down..."

```rs
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // force the thread to stop execution for some time
        }
    }); // this may not print all values, because the below loop might finish more quickly

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

### Waiting for All Threads to Finish Using `join` Handles

To force the main thread to wait for the spawned thread to finish
- save the (`JoinHandle`) value returned from `thread::spawn` to a variable
- call `join()` on that variable

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| { // handle is a JoinHandle
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // main will not exit until handle finishes
}
```

`handle.join()` causes the main thread to _block_ until `handle` finishes.

Note that calling `join` _before_ the second `for` loop will cause `handle`'s thread to run through all 9 numbers before the main thread's loop can start printing.

### Using `move` Closures with Threads

Use `move` in conjunction with `thread::spawn` to allow the spawned thread to take ownership of the values passed to it.

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {           // Rust infers that we want a reference to v here
        println!("Here's a vector: {:?}", v); // because we only need a reference to v to print it
    });

    drop(v); // but that reference could possibly outlive the value it's referencing
             // so this example won't compile

    handle.join().unwrap();
}
```

With `move`

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // now this thread owns v
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // this program still won't compile, but for a different reason
             // now that we've moved v into the new thread, we cannot drop it here

    handle.join().unwrap();
}
```

## Using Message Passing to Transfer Data Between Threads

From the Golang docs: "Do not communicate by sharing memory; instead, share memory by communicating."

Rust provides for message-sending concurrency with _channels_. A channel allows data to be sent from one thread to another.

> "A channel has two halves: a transmitter and a receiver. ... A channel is said to be _closed_ if either the transmitter or receiver half is dropped."

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    // create a new channel using `mpsc::channel`
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();   // (transmitter, receiver)

    thread::spawn(    // create a new thread
      move || {       // move tx into this new thread
        let val = String::from("hi");
        tx
          .send(val)  // tx sends one String to rx
          .unwrap();  // send returns a Result<T, E> which returns an error if rx has been dropped      
      }
    ); // "The spawned thread needs to own the transmitter to be able to send messages through the channel."

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

> "...a channel can have multiple _sending_ ends ["multiple producer"] that produce values but only one _receiving_ end ["single consumer"] that consumes those values. Imagine multiple streams flowing together into one big river: everything sent down any of the streams will end up in one river at the end."

`Receiver#recv()` blocks until it receives a value (returns `Ok`), or the transmitter closes, after which it will return an `Err`.

`Receiver#try_recv()` doesn't block. It will immediately return an `Ok` if it has a value available to return, otherwise, it will return an `Err`.

`try_recv()` is useful in a loop. The main thread could be doing other work, and check if it has received a value at regular intervals, rather than blocking until it has one.

### Channels and Ownership Transference

`send()` takes ownership of its argument

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi"); // This example will not compile because
        tx.send(val).unwrap();        //   tx.send() takes ownership of val here
        println!("val is {}", val);   //   we're trying to borrow val here
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### Sending Multiple Values and Seeing the Receiver Waiting

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),     // Running this example shows these
            String::from("from"),   // messages arriving one at a time,
            String::from("the"),    // with a 1 second interval in between each
            String::from("thread"), // because of the 1 secod thread:sleep() below
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // treat rx as an iterator
        println!("Got: {}", received);
    } // the iteration will stop when the transmitter closes
}
```

### Creating Multiple Producers by Cloning the Transmitter

```rs
    // --snip--

    let (tx, rx) = mpsc::channel(); // single receiver, rx

    let tx1 = tx.clone();
    thread::spawn(move || { // first spawned thread
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // transmitter 1: tx1
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || { // second spawned thread
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // transmitter 2: tx
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // rx receives from both tx and tx1
        println!("Got: {}", received);
    }

    // --snip--
```

> "Channels can only send values of a single type"

...so you can't send a `String` and an integer down the same channel.

## Shared-State Concurrency

> "...let’s look at mutexes, one of the more common concurrency primitives for shared memory."

### Using Mutexes to Allow Access to Data from One Thread at a Time

> "_Mutex_ is an abbreviation for _mutual exclusion_... allows only one thread to access some data at any given time."

A mutex guards the data it holds with a lock. A thread signals that it wants to acquire the lock, and the mutex grants access as long as no other thread currently has the lock.

When using a mutex, you must

1. remember to acquire the lock before using the data
2. remember to release the lock when you're finished so that other threads can use the data

#### The API of `Mutex<T>`

```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m // the type system ensures that we must acquire a lock
          .lock()       // blocks until we acquire the lock, returns a LockResult<MutexGuard<i32>>
          .unwrap();    // unwraps the LockResult, returning a MutexGuard<i32>

        *num = 6;
    } // mutex lock should be released here

    println!("m = {:?}", m);
}
```

> "The call to `lock` would fail if another thread holding the lock panicked. In that case, no one would ever be able to get the lock, so we’ve chosen to `unwrap` and have this thread panic if we’re in that situation."

Book seems to be out of date here: "releases the lock automatically when a `MutexGuard` goes out of scope, which happens at the end of the inner scope". This doesn't seem to be the case.

#### Sharing a `Mutex<T>` Between Multiple Threads

```rs
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 { // This example doesn't compile because
        let handle = thread::spawn(move || { // counter is moved into the first thread here
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

#### Multiple Ownership with Multiple Threads

```rs
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 { // This example also doesn't compile because
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || { // "`Rc<Mutex<i32>>` cannot be sent between threads safely"
            let mut num = counter.lock().unwrap(); // the trait `Send` is not implemented for `Rc<Mutex<i32>>`

            *num += 1;
        });
        handles.push(handle);
    }

    // --snip--
}
```

`Rc<T>` doesn't update its reference count atomically, and so is not thread safe.

#### Atomic Reference Counting with `Arc<T>`

`Arc` == "atomically reference counted"

Atomics are thread-safe, but at a performance cost. So we only use `Arc<T>` over `Rc<T>` (and only use atomics in general) when necessary.

```rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

The above example will finally print "Result: 10".

Note that atomic types can be used directly, rather than wrapping primitive types in mutexes. See: https://doc.rust-lang.org/std/sync/atomic/index.html

### Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

> "You might have noticed that `counter` is immutable but we could get a mutable reference to the value inside it; this means `Mutex<T>` provides interior mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`."

Similarly to how `Rc<T>` can be used to create _reference cycles_, `Mutex<T>` can be used to create _deadlocks_. So you must verify the logical behaviour of your program, which the compiler can't do for you.

## Extensible Concurrency with the `Sync` and `Send` Traits

Channels and mutexes are not part of the Rust language itself, but part of the standard library.

However, the `std::marker` traits `Sync` and `Send` _are_ a part of the language itself.

### Allowing Transference of Ownership Between Threads with `Send`

Almost every Rust type implements `Send`, which indicates that ownership of values of that type can be transferred between threads.

As an example, `Rc<T>` does _not_ implement `Send` because its reference counts are not updated atomically. The Rust compiler threw an error in the example above where we tried to `move` an `Rc<T>` into a thread.

> "Any type composed entirely of `Send` types is automatically marked as `Send` as well. Almost all primitive types are `Send`, aside from raw pointers, which we’ll discuss in Chapter 19."

### Allowing Access from Multiple Threads with `Sync`

> "The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread. Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`."
>
> "The smart pointer `Rc<T>` is also not `Sync` for the same reasons that it’s not `Send`. The `RefCell<T>` type (which we talked about in Chapter 15) and the family of related `Cell<T>` types are not `Sync`. The implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe. The smart pointer `Mutex<T>` is `Sync` and can be used to share access with multiple threads as you saw in the “Sharing a Mutex<T> Between Multiple Threads” section."

### Implementing `Send` and `Sync` Manually Is Unsafe

`Send` and `Sync` are _marker traits_ and don't have any methods to implement.

Manually implementing these traits requires unsafe Rust code. See [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) for more information.

## Summary

Crates evolve more quickly than the standard library, or the Rust language itself.

> "The type system and the borrow checker ensure that the code using these solutions won’t end up with data races or invalid references. Once you get your code to compile, you can rest assured that it will happily run on multiple threads without the kinds of hard-to-track-down bugs common in other languages."