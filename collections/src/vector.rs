#![allow(unused)]
#![allow(dead_code)]

pub fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn invalid_mutability() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("The first element is: {first}");
}

fn iterate_over_vector() {
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    // it's also possible to iterate over mutable references
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        // v2.push(100); // error[E0499]: cannot borrow `v2` as mutable more than once at a time
        *i += 50; // We have to use the * dereference operator to get to the value in i before we can use the += operator.
    }
}

// Using an enum to Store Multiple Types

fn using_enum_to_store_multiple_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// Dropping a Vector Drops Its Elements

fn dropping_vector_drops_elements() {
    let _v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here
