use core::panic;
use std::{fs::File, error};
use std::io::{ErrorKind, self, Read};
use std::io;
use std::fs;

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // execute cargo run and see output
    // v[99];

    // handle_error();

    // unwrap_and_expect();

    propagate_error();
}

fn handle_error() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}

fn unwrap_and_expect() {
    // let f = File::open("hello_again.txt").unwrap();

    // expect is better than unwrap?
    let f = File::open("hello_again.txt").expect("Failed to open hello.txt");
}

fn propagate_error() {
    match read_username_from_file() {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("Error when read username: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
