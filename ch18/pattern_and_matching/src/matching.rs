fn match_literal() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_named_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // y in match shadow the one out of shadow
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn match_or() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_range() {
    // only can use in numeric type and char type
    let x = 5;

    match x {
        1 ..= 5 => println!("one through five"), // match [1, 5]
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("early ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destruct_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    // same as above but easier
    let Point { x, y } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn destruct_enum() {
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::_Quit => println!("The Quit variant has no data to destructure."),
        Message::_Move { x, y } => println!(
            "Move in the x direction {} and in the y direction {}", x, y
        ),
        Message::_Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to RGB({}, {}, {})", r, g, b
        )
    }
}

fn desturct_nest_enum_or_struct() {
    enum Color {
        _Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::_Rgb(r, g, b)) => println!(
            "Change the color to RGB({}, {}, {})", r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to HSV({}, {}, {})", h, s, v
        ),
        _ => ()
    }
}

fn destruct_pattern() {
    destruct_struct();
    destruct_enum();
    desturct_nest_enum_or_struct();
}

fn ignore_first_parameter(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignore_fn_parameter() {
    ignore_first_parameter(3, 4);
}

fn ignore_in_other_pattern() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn difference_between_underscore_and_name_prefix_underscore() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    // _x will actully bind value to _x, but _ won't
    // so it will cause s moved
    // if let Some(_x) = s {
    //     println!("found a string");
    // }

    println!("{:?}", s);
}

fn ignore_remaining_parts() {
    // struct
    struct Point3 {
        x: i32,
        _y: i32,
        _z: i32,
    }

    let origin = Point3 { x: 0, _y: 0, _z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    // tuple
    let numbers = (2, 4, 6, 8, 10);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn ignore_pattern() {
    ignore_fn_parameter();
    ignore_in_other_pattern();
    difference_between_underscore_and_name_prefix_underscore();
    ignore_remaining_parts();
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 4 =>  println!("num less than five: {}", x),
        Some(x) => println!("{}", x),
        None => ()
    }
}

fn at_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3 ..= 7 } => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello { id: 10 ..= 12 } => {
            println!("Found an id in another range");
            // can't access id directly
            // println!("Found an id in another range {}", id);
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        },
    }
}

pub fn run() {
    match_literal();
    match_named_variable();
    match_or();
    match_range();
    destruct_pattern();
    ignore_pattern();
    match_guard();
    at_binding();
}