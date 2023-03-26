// Concise Control Flow with if let
// https://doc.rust-lang.org/book/ch06-03-if-let.html

// Choosing between match and if let depends on what you’re doing in
// your particular situation and whether gaining conciseness is an appropriate
// trade-off for losing exhaustive checking.

mod r#match;
use crate::r#match::Coin;

fn main() {
    using_match();
    using_if_let();
}

fn using_match() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

fn using_if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

// fn count_all_non_quarter_coin_using_match() {
//     let mut count = 0;
//     let coin = Coin::Quarter;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
// }

// fn count_all_non_quarter_coin_using_if_let() {
//     let mut count = 0;
//     let coin = Coin::Quarter;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count += 1;
//     }
// }
