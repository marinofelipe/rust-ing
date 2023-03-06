// 4.3 - The slice type
// https://doc.rust-lang.org/book/ch04-03-slices.html

// fn first_word(s: &String) -> ?

fn error_prone_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn example_of_usize_only_being_messy() {
    let mut s = String::from("hello world");

    let _word = error_prone_first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

// It would be even more complex and brittle to track start and end indexes
//
// "Now we’re tracking a starting and an ending index,
// and we have even more values that were calculated from data in a
// particular state but aren’t tied to that state at all.
// We have three unrelated variables floating around that need to be kept in sync."
// - The Rust Book
//
// fn second_word(s: &String) -> (usize, usize) {
// }

fn _string_slices() {
    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];
}

fn _ranges() {
    let s = String::from("hello");

    // these are the same
    let _slice = &s[0..2];
    let _slice = &s[..2];

    let len = s.len();

    // and so are these
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // and these
    let _slice = &s[0..len];
    let _slice = &s[..];
}

fn _compile_safe_first_world_example() {
    let mut s = String::from("hello world");

    let _word = first_word(&s);

    s.clear(); // error!

    // println!("the first word is: {}", word);
}

pub fn slice_example() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
}

// fn first_word(s: &String) -> &str {

// optimized to work with &String values and &str values.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
