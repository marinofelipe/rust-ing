#![allow(unused)]
#![allow(dead_code)]

mod r#match;
use crate::r#match::match_bind_to_values;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    store_values_in_struct();
    with_associated_values();
    with_different_associated_values_per_case();
    enum_method();

    // option
    option_examples();
    option_type_safety();

    match_bind_to_values();
}

fn route(ip_kind: IpAddrKind) {
    println!("Routed to IP: {:?}", ip_kind)
}

fn store_values_in_struct() {
    #[allow(dead_code)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// enum with associated values

enum IpAddr {
    V4(String),
    V6(String),
}

fn with_associated_values() {
    let _home = IpAddr::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr::V6(String::from("::1"));
}

// differently than structs enums allow each case to have
// different associated values (such as in Swift)

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(unused_variables)]
fn with_different_associated_values_per_case() {
    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));
}

// stdlib has a definition that we can use
// https://doc.rust-lang.org/std/net/enum.IpAddr.html
// which embodies structs as the content for each case

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// -------------------------
// With embedded types in it
// which becomes associated and type nested to the enum (as a namespace)

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// the following structs could hold the same data

#[allow(dead_code)]
struct QuitMessage; // unit struct

#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct WriteMessage(String); // tuple struct

#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But if we used the different structs, each of which has its own type,
// we couldn’t as easily define a function to take any of these kinds of messages
// as we could with the Message enum, which is a single type.

// -----------------------
// enums also support methods

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn enum_method() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// ------------------------
// The Option enum
//

// very similar to Swift, it's also an enum
// that hold a generic type

#[allow(unused_variables)]
fn option_examples() {
    let some_number = Some(5);
    let some_char = Some('e');

    // here the type has to be made explicit, otherwise
    // Rust can't infer from None (again, a type system similar to Swift's)
    let absent_number: Option<i32> = None;
}

fn option_type_safety() {
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y; => error[E0277]: cannot add `Option<i8>` to `i8`
}
