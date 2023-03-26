// 6.2 - The Rust Book
// https://doc.rust-lang.org/book/ch06-02-match.html

fn main() {
    match_bind_to_values();
    match_control_flow();

    catch_all_example();
    catch_all_example_2();
    catch_all_example_3();
}

pub fn match_control_flow() {
    println!("bla");
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn _value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn _value_in_cents2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns that bind to values

pub fn match_bind_to_values() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents3(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alaska) => {
                println!("It's god damn cold in Alaska!");
                25
            }
            Coin::Quarter(UsState::Alabama) => {
                println!("It's Bama, baby!!");
                25
            }
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents3(Coin::Penny);
    value_in_cents3(Coin::Quarter(UsState::Alabama));
    value_in_cents3(Coin::Quarter(UsState::Alaska));
}

// ------------------------------
// Match with Option<T>
// https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont

fn using_option_with_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// bind values
fn catch_all_example() {
    let dice_roll = 9;

    match dice_roll {
        // Rust warns when a catch all is used at first
        // arms(cases) are evaluated by other
        // other => move_player(other),
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// don't bind value and take action
fn catch_all_example_2() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

// do nothing
fn catch_all_example_3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
