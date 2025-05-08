#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;

fn basics() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values
    let team_name = String::from("Blue");
    // `copied` to get an Option<i32> rather than an Option<&i32>
    // unwrap_or to set score to zero if scores doesn’t have an entry for the key (if Option is None)
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

fn iterate_over_key_value_pair() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn ownership() {
    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name} and {field_value}"); // error[E0382]: borrow of moved value: `field_name`
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn overriding_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}

fn adding_only_if_not_present() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

fn conditional_update() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // the mutable value inserted or returned is incremented by 1
    }

    println!("{map:?}");
}
