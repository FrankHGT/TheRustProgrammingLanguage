fn main() {
    let mut s = String::new();

    let data = "initial contents";

    s = data.to_string();
    s = "initial contents".to_string();

    println!("{}", s);

    update_string();

    combine_strings();

    index_string();

    iterate_string();
}

fn update_string() {
    let mut s = String::from("foo");
    // for str
    s.push_str("bar");
    println!("{}", s);

    // for single char
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}

fn combine_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // s3 take s1 ownership, and get unmutable reference for s2;
    // move s1 to s3, copy s2 to s3
    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn index_string() {
    let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("Hola").len();
    println!("len of Hola: {}", len);

    let hello = "Здравствуйте";
    println!("len of Здравствуйте: {}", hello.len());

    let s = &hello[0..4];
    println!("{}", s);
}

fn iterate_string() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
