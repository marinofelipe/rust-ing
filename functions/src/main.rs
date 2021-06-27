fn main() {
    another_function(5, 6);
    // let x = (let y = 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    // 5; statements don’t evaluate to a value.
    5
}

// this is a comment
// yes, same as usual
fn plus_one(x: i32) -> i32 {
    x + 1
}