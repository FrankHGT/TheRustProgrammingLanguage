use std::option;

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

// method on enum
impl Message {
    fn call(&self) {
        println!("called an enum variant method");
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(IpAddrKind::V6(String::from("::1")));

    let msg = Message::Write(String::from("hello"));
    msg.call();

    // option test
    option_test();
}

fn route(_ip_type: IpAddrKind) {

}

fn option_test() {
    let some_number = Some(2);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    // compile error: no type for option
    // let absent_number = None;


    // T and Option<T> are different types
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // compile error: mismatched types
    // let sum = x + y;
}
