use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::io::empty;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101); // 1..=100
    let mut guess: u32;
    let mut result: Ordering;

    loop {
        guess = match ask_for_guess() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease type a number!");
                continue;
            }
        };
        result = read_result(&guess, &secret_number);
        show_result(&result);

        if result == Ordering::Equal {
            break;
        }
    }
}

fn ask_for_guess() -> Result<u32, io::Empty> {
    println!("\nGuess the number!\nPlease input your guess:");

    let mut guess = String::new();

    io::stdin() // std::io::stdin() would work as well - without the need to import/use std::io
        .read_line(&mut guess)
        .expect("Failed to read line."); // ok to crash in this program

    let guess: u32 = match guess
        .trim() // trim out any leading and trailing whitespaces that the user may have added + the \n that gets added
        .parse() { // transform into the target type via inference + handle error
        Ok(num) => num,
        // TODO: Should find a better way to convert from ParseIntError -> io:Error.
        // Tried a custom From<> impl but without success. Let's see how it goes later!
        Err(_) => return Err(empty()),
    };

    return Ok(guess);
}

fn read_result(guess: &u32, secret_number: &u32) -> Ordering {
    println!("\nYou guessed: {}", guess);
    return guess.cmp(&secret_number);
}

fn show_result(ordering: &Ordering) {
    match ordering {
        Ordering::Less => println!("\nToo small!"),
        Ordering::Greater => println!("\nToo big!"),
        Ordering::Equal => println!("\nNice job! You won!")
    }
}