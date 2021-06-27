fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number { // expected `bool`, found integer
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // multiple conditions

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // if let - type mismatch
    // let number = if condition { 5 } else { "six" }; // error[E0308]: `if` and `else` have incompatible types
    // println!("The value of number is: {}", number);

    // loops();
    loop_that_returns();
    while_loop();
    for_loop();
}

// fn loops() {
//     loop {
//         println!("again!");
//     }
// }

fn loop_that_returns() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    println!("The counter is {}", counter);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // more sage and concise approach with for
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}