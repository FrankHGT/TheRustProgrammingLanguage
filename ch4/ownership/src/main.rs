fn main() {
    // scope
    {
        // 还未声明，不可用
        let s = "hello"; // 可用
        // 相关操作
        println!("{} world", s);
    }   // 作用域结束，不可用

    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    // use after assign
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    // error
    // println!("{}, world", s1);
    println!("{}, world", s2);
    println!("{}, world s3", s3);

    let s = String::from("hello"); // s进入作用域

    takes_ownership(s); // s被move到了函数内
    // println!("{}", s);           // s不再有效，使用会编译错误

    let x = 5;              // x进入作用域

    makes_copy(x); // x被copy到了函数内
    println!("{}", x);

    let s1 = gives_ownership();

    let s3 = takes_and_gives_back(s1);

    // println!("s1 not avalible {}", s1);
    println!("s3 is avalible {}", s3);

    // reference
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    // can't use s1, must prefix with &, this is difference with c++
    // let len = calculate_length(s1);     // error
    println!("The length of '{}' is {}.", s1, len);

    // you must pass value as &mut s1,
    // if you want to change something, you must let owner know!
    // change(&s1);

    // mutable reference
    change(&mut s1);

    let mut s = String::from("hello");
    let r2 = &mut s;
    // can't borrow as mutable more than once at a time
    // let r3 = &mut s;

    r2.push_str(", r2");
    // r2.push_str(", r3");

    println!("s : {}", s);

    // unmutable reference, mutable reference can't exist at the same time
    let mut s = String::from("hello");

    let r1 = &s;
    // let r2 = &mut s;

    println!("r1: {}", r1);

    string_slice();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// reference
fn calculate_length(s: &String) -> usize {
    s.len()
}

// can't change borrowed data
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangling reference
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn string_slice() {
    let mut s = String::from("hello world");

    let index = first_word_end(&s);

    println!("The first word end index is: {} s: {}", index, s);

    // we can clear it, cause index isn't a reference
    s.clear();
    // will panic, cause s has been cleared
    // println!("end of first word: {}", s.as_bytes()[index - 1] as char);

    s = String::from("frank hgt");

    let word = first_word(&s);

    // 字符串切片word是对s的不可变引用
    // s.clear()是对s的可变引用
    // 不能同时拥有一个变量的可变与不可变引用
    // s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_end(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}