
fn main() {
    // compile error, cause x is unmutable by default
    // let x = 5;
    let mut x = 5;

    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("The maximum number of points is: {}", MAX_POINTS);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructing
    let (_, y, _) = tup;

    println!("The value of y is: {}", y);
    // index access
    println!("The first element of tup is: {}", tup.0);

    // array
    let a = [1, 2, 3, 4, 5];

    let index = 4;
    let element = a[index];

    println!("The value of element is: {}", element);

    another_function(5);

    // expression
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // no ; end of line, or expression will turn to statement
    };

    println!("The value of x is: {} y is: {}", x, y);

    println!("Result of five(): {}", five());

    println!("Result of plus_one(5): {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("Another function was called. x: {}", x);
}

fn five() -> i32 { 
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
