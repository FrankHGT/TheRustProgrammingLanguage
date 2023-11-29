fn test(x: i32) -> i32 {
    x
}

// you can directly return a function pointer
// cause is basiclly a pointer, it's Sized
fn returns_fn() -> fn(i32) -> i32 {
    test
}

// but you can't return Fn, cause it's a triat
// trait has dynamiclly size, it's not Sized
// but a function's return type need to be Sized
// cause it will store in stack
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    // Box::new(test)

    Box::new(|x| x)
}

pub fn run() {
    let func = returns_fn();
    println!("x is: {}", func(1));

    let closure = returns_closure();
    // automically dereference
    println!("x is: {}", closure(1));
}