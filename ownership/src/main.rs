// the stack and the heap

// pushing to the STACK is faster, because the allocator doesn't have to find space to store,
// it's always in the top of the stack

// while in the HEAP a certain amount of space is required to the allocator, which then has
// to find available big enough space

// accessing data in the HEAP is slower because requires following a pointer to get there,
// while in the STACK it's always in the top

// "a processor can do its job better if it works on data that’s close to other data (as it is on the stack)
// rather than farther away (as it can be on the heap)." - Rust book, https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// "When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap)
// and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack."
// - Rust book, https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// >> Ownership Rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

mod slices;
use crate::slices::example_of_usize_only_being_messy;
use crate::slices::slice_example;

fn main() {
    {
        // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    ownership();
    ownership_adn_fns();
    return_values_and_scope();
    reference();
    attempt_to_modify_borrowed_value();
    mutable_borrow();
    multiple_mutable_references();
    multi_mutable_references_in_distinct_scopes();
    combine_mutable_and_imutable_references();
    scope_mutable_and_imutable_refs();
    // let reference_to_nothing = dangle();
    no_dangle();
    example_of_usize_only_being_messy();
    slice_example();
}

fn ownership() {
    // let s = String::from("hello");

    // String::from implementation requests the HEAP memory it needs.
    // This is pretty much universal in programming languages.
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

fn ownership_adn_fns() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope() {
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn reference() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

//// Borrowing ////

fn attempt_to_modify_borrowed_value() {
    let s = String::from("hello");

    change(&s);
}

fn change(_some_string: &String) {
    // some_string.push_str(", world"); => error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}

fn mutable_borrow() {
    let mut s = String::from("hello");

    change_2(&mut s);
}

fn change_2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_mutable_references() {
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // => error[E0499]: cannot borrow `s` as mutable more than once at a time

    // println!("{}, {}", r1, r2);
}

fn multi_mutable_references_in_distinct_scopes() {
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}

fn combine_mutable_and_imutable_references() {
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM / error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("{}, {}, and {}", r1, r2, r3);
}

fn scope_mutable_and_imutable_refs() {
    // The scopes of the immutable references r1 and r2 end after the println!
    // where they are last used, which is before the mutable reference r3 is created.
    // These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference
    // is no longer being used at a point before the end of the scope.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn dangle() -> &String { // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// side note: Now the name Borrowing makes a lot of sense :D //
