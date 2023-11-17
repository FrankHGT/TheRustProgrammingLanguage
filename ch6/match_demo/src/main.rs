#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Value of a Quarter: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("Value of a Penny: {}", value_in_cents(Coin::Penny));

    match_option();

    match_wildcard();

    if_let();
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn match_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1),
    }
}

fn match_wildcard() {
    print!("match \"_\" wildcard test, u8 is ");
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        _ => println!("default"),
    }
}

fn if_let() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
