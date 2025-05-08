#![allow(unused)]
#![allow(dead_code)]

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];
}

// env var RUST_BACKTRACE=1 for enabling backtrace. `full` for a complete one.

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // using closures instead of match
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // using unwrap to automatically panic
    let greeting_file = File::open("hello.txt").unwrap();

    // with expect for specific panic messages
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn propagating_errors() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // shortcut
    fn read_username_from_file_shorter() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // even shorter
    fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // eeeeven shorter
    fn read_username_from_file_simpler() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// `main` can have a Result return
fn main2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
