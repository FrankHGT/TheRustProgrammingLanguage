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
}

fn route(_ip_type: IpAddrKind) {

}
