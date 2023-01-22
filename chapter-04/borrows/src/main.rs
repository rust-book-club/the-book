fn main() {

    let s1 = String::from("s1");
    let mut s2 = String::from("s2");

    borrow(&s1);
    // borrow_mutable(&mut s1); // cannot borrow as mutable
    take_ownership(s1);

    borrow(&s2);
    borrow_mutable(&mut s2);
    take_ownership(s2);
}

// case 1: an immutable object that you take ownership of
fn take_ownership(s: String) {
    println!("Took ownership of {s}")
}

// case 2: an immutable object that you borrow
fn borrow(s: &String) {
    // Cannot borrow immutable local variable `s` as mutable
    // s.push_str(" plus this");
    println!("Borrowed {s}")
}

// case 3: a mutable object that you take ownership of
//   call take_ownership(), passing in a mutable object

// case 4: a mutable object that you borrow, but are not allowed to mutate *
//   call borrow(), passing in a mutable object

// case 5: a mutable object that you borrow, and are allowed to mutate *
fn borrow_mutable(s: &mut String) {
    s.push_str(" plus this");
    println!("Borrowed {s}")
}

// case 6: an immutable object that you borrow, and are allowed to mutate
//   call borrow_mutable(), passing in an immutable object