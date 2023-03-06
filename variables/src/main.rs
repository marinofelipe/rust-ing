const MAX_POINTS: u32 = 100_000;

fn main() {
    const LOCAL_CONST: &str = "100_000";
    println!("{}", MAX_POINTS.to_string() + LOCAL_CONST);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let _x = x * 2;
    let x = 3;
    println!("The value of x is: {}", x);

    // shadowing with different type

    let spaces = "   ";
    let _spaces = spaces.len();

    // of course changing the type in the fly is not possible, given that Rust is a statically typed language
    let mut _spaces = "   ";
    // spaces = spaces.len(); // error[E0308]: mismatched types
}

fn _scalar_types() {
    // signed and unsigned integers
    let _x: i8 = 1; // -128 -> +127
    let _x: i16 = 1;
    let _x: i32 = 1;
    let _x: i64 = 1;
    let _x: i128 = 1;

    let _x: u8 = 1; // 0 -> 255
    let _x: u16 = 1;
    let _x: u32 = 1;
    let _x: u64 = 1;
    let _x: u128 = 1;

    // float
    let _x = 2.0; // f64 - double precision - aka double
    let _y: f32 = 3.0; // f32 - single precision
}

fn _numeric_operations() {
    // remainder
    let _remainder = 43 % 5;
}

fn _tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching - destructuring the tuple
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // accessing by the index also works - as in Swift
    let _five_hundred = x;
    let _six_point_four = y;
    let _one = z;
}

fn _arrays() {
    let _a = [1, 2, 3, 4, 5];

    // different from vectors - that can grow in size
    // arrays are generally used in cases where the data is static
    // e.g. - months
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // can also be declared with a shorthand that takes a value to repeat (3 in this case)
    // and the size of the array (5)
    let _a = [3; 5]; // let a = [3, 3, 3, 3, 3]

    // Nice things with e.g. accessing a invalid index of array is that
    // Rust is more safe than other low level languages,
    // since the invalid memory is not accessed, but instead the program
    // immediately exits when it panics!
}
