use rand::Rng;
use std::io::empty;
use std::{cmp::Ordering, io};
use std::{error::Error, fmt};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101); // 1..=100
    let mut guess: Guess;
    let mut result: Ordering;

    loop {
        guess = match ask_for_guess() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        result = read_result(&guess.value(), &secret_number);
        show_result(&result);

        if result == Ordering::Equal {
            break;
        }
    }
}

fn ask_for_guess() -> Result<Guess, io::Empty> {
    println!("\nGuess the number!\nPlease input your guess:");

    let mut value = String::new();

    io::stdin() // std::io::stdin() would work as well - without the need to import/use std::io
        .read_line(&mut value)
        .expect("Failed to read line."); // ok to crash in this program

    let value: i32 = match value
        .trim() // trim out any leading and trailing whitespaces that the user may have added + the \n that gets added
        .parse()
    {
        // transform into the target type via inference + handle error
        Ok(num) => num,
        // TODO: Should find a better way to convert from ParseIntError -> io:Error.
        // Tried a custom From<> impl but without success. Let's see how it goes later!
        Err(_) => return Err(empty()),
    };

    return match Guess::new(value) {
        Ok(guess) => Ok(guess),
        Err(error) => {
            println!("{}", error);
            return Err(empty());
        }
    };
}

fn read_result(guess: &u32, secret_number: &u32) -> Ordering {
    println!("\nYou guessed: {}", guess);
    return guess.cmp(&secret_number);
}

fn show_result(ordering: &Ordering) {
    match ordering {
        Ordering::Less => println!("\nToo small!"),
        Ordering::Greater => println!("\nToo big!"),
        Ordering::Equal => println!("\nNice job! You won!"),
    }
}

// Chapter 9 - Creating a type that validates the value internally
// N.B.: I took the chance to further improve it by playing around with
// not panickingsd
struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: i32) -> Result<Guess, GuessError> {
        return match value {
            1..=100 => Ok(Guess {
                value: value as u32,
            }),
            _ => Err(GuessError::InvalidGuess(value)),
        };
    }

    // would only be needed if it was in another mod, with `value` not being pub
    fn value(&self) -> u32 {
        self.value
    }
}

#[derive(Debug)]
enum GuessError {
    InvalidGuess(i32),
}

impl Error for GuessError {}

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuessError::InvalidGuess(value) => {
                write!(f, "Guess value must be between 1 and 100, got {}.", value)
            }
        }
    }
}
