fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // number is expression for integer in rust, and Rust will
    // not automatically convert it to boolean, so it will give error
    // if number {
    //     println!("test")
    // }

    let number = 6;

    if number % 4 == 0 {
        println!("number if divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // "if" is an expression
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // all arms in if else must have same type
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
}
