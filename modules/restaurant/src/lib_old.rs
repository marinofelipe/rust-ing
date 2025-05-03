#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// super
//
// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super
//
// We can construct relative paths that begin in the parent module,
// rather than the current module or the crate root, by using super at the start of the path.
// This is like starting a filesystem path with the .. syntax.
//

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// structs and enums as public
//
// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public
//
// We can construct relative paths that begin in the parent module,
// rather than the current module or the crate root, by using super at the start of the path.
// This is like starting a filesystem path with the .. syntax.
//

mod back_of_house_2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house_2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house_2::Appetizer::Soup;
    let order2 = back_of_house_2::Appetizer::Salad;
}

// the use keyword
//
// https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
//
//

use crate::front_of_house::hosting;

pub fn eat_at_restaurant_3() {
    // Absolute path
    hosting::add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // super
        super::hosting::add_to_waitlist();

        // or via mod internal import (line 126)
        hosting::add_to_waitlist();
    }
}

// non-idiomatic fns usage
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant_4() {
    add_to_waitlist(); // it's not clear where add_to_waitlist does comes from
}

// however, it's idiomatic for types
use std::collections::HashMap;

fn example() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Bringing two types with the same name ("Result")
// In this casem, the parent module is used (in "use")
use std::fmt;
use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Bringing two types with the same name ("Result")
// In this casem, a custom name is provided to one of them
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// Re-exporting
//
// Before this change, external code would have to call the add_to_waitlist function
// by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module,
// external code can now use the path restaurant::hosting::add_to_waitlist() instead.

// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_5() {
    hosting::add_to_waitlist();
}

// Nested paths

// This
// use std::{cmp::Ordering, io};

// Instead of
// use std::cmp::Ordering;
// use std::io;

// another example

// This, where `self` brings `std::io` itself
// use std::io::{self, Write};

// Instead of
// use std::io::{self, Write};

// The Glob Operator

// If we want to bring all public definitions of a module, we can use the "*" operator

// it brings all public items defined in std::collections into the current scope
use std::collections::*;

// > **Be careful** when using the glob operator!
// Glob can make it harder to tell what names are in scope and where a name used
// in your program was defined.

// The glob operator is often used when testing
// to bring everything under test into the tests module;
