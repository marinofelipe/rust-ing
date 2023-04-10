// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
//
// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// Modules
//
// Cheat Sheet => https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet
//

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus { size: 6 };
    println!("I'm growing {:?}!", plant);
}

// !!!!
//
// Items in a parent module can’t use the private items inside child modules,
// but items in child modules can use the items in their ancestor modules.
// This is because child modules wrap and hide their implementation details,
// but the child modules can see the context in which they’re defined.
//
// !!!!

// If you plan on sharing your library crate so other projects can use your code,
// your public API is your contract with users of your crate that determines how they can interact with your code.
// There are many considerations around managing changes to your public API to make it easier for people to depend on your crate.
// These considerations are out of the scope of this book; if you’re interested in this topic,
// see The Rust API Guidelines (https://rust-lang.github.io/api-guidelines/).

// super
//
// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super
//
// We can construct relative paths that begin in the parent module,
// rather than the current module or the crate root, by using super at the start of the path.
// This is like starting a filesystem path with the .. syntax.
//

#[allow(dead_code)]
fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// strtucts and enums as public
//
// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public
//
// We can construct relative paths that begin in the parent module,
// rather than the current module or the crate root, by using super at the start of the path.
// This is like starting a filesystem path with the .. syntax.
//

#[allow(dead_code)]
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

#[allow(unused_variables)]
pub fn eat_at_restaurant() {
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
