struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // structs can have their fields declared in any order
    let _user2 = User {
        active: false,
        email: String::from("email"),
        sign_in_count: 2,
        username: String::from("someusername12"),
    };

    acccess_struct_value();

    let _user_x = build_user(String::from("email@a.com"), String::from("username"));

    update_syntax();
}

fn acccess_struct_value() {
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    // - The Rust book
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    print!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // not using it
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // using it

    // Note that the struct update syntax uses = like an assignment;
    // this is because it moves the data,
    // - The Rust book
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    print!("{}", user1.active);

    if user1.active {
        print!("{}", user1.sign_in_count);
    }
}

fn _tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn _unit_like_structs() {
    struct AlwaysEqual;

    let _subject = AlwaysEqual;
}
