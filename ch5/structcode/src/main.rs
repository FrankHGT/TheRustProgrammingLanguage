use std::sync::Arc;

fn main() {
    let mut user1 = User {
        email: String::from("huguantingfrank@outlook.com"),
        username: String::from("frank hgt"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("huguantingfrank@outlook.com");

    let user2 = build_user(String::from("test@test.com"), String::from("test"));

    println!("user2's username: {}", user2.username);

    // struct update sematic syntax
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user3's username: {}", user3.username);

    tuple_struct();
    empty_struct();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand 字段初始化简写
        username,
        active: true,
        sign_in_count: 1,
    }
}

// tuple struct
fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // black and origin is different types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black is R:{} G:{} B:{}", black.0, black.1, black.2);
    println!("origin is at {}, {}, {}", origin.0, origin.1, origin.2);
}

// empty struct 
fn empty_struct() {
    struct Empty;
    let e = Empty;
}
