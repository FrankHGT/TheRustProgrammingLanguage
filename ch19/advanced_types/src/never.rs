fn _bar() -> ! {
    panic!("panic!")
}

pub fn run() {
    // expression return !
    // 1. continue
    // 2. panic!
    // 3. loop

    let x = Some(5);

    // ! can convert to any type, this's why you can 
    // use it in the end of match arms, and not trigger
    // compile error!
    let x = match x {
        Some(v) => v,
        None => panic!("dududu"),
    };

    println!("x is: {}", x);
}