pub fn run() {
    let some_option_value = Option::Some(5);

    // let need a irrefutable pattern, but Some(x) is refutable
    // let Some(x) = some_option_value;

    if let Some(x) = some_option_value {
        println!("x: {}", x);
    }

    // if let need a refutable pattern, but 5 is irrefutable
    // if let x = 5 {
    //     println!("x: {}", x);
    // }
}