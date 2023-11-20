use std::{fs::File, error};

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // execute cargo run and see output
    // v[99];

    handle_error();
}

fn handle_error() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
