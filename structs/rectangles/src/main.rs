fn main() {
    area_as_ints();
    area_as_tuple();
    area_as_structs();
    using_dbg_macro();
    using_method();
    using_method_named_as_prop();
    can_hold_usage();
    using_the_custom_constructor();
}

fn area_as_ints() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn area_as_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_as_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("The rectangle is {:?}", rect1);

    println!("The rectangle is {:#?}", rect1);

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

fn using_dbg_macro() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// method syntax

impl Rectangle {
    // We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership,
    // and we just want to read the data in the struct, not write to it.
    // If we wanted to change the instance that we’ve called the method on as part of what the method does,
    // we’d use &mut self as the first parameter.
    // Having a method that takes ownership of the instance by using just self as the first parameter is rare;
    // this technique is usually used when the method transforms self into something else and you want to prevent the caller
    // from using the original instance after the transformation.
    // https://doc.rust-lang.org/book/ch05-03-method-syntax.html
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn using_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// methods can be named exactly the same as properties
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn using_method_named_as_prop() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// multiple params

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn can_hold_usage() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Associated fns that are not methods (don't receive self)

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn using_the_custom_constructor() {
    let rect1 = Rectangle::square(20);

    println!("rect1 {:#?}", rect1);
}
