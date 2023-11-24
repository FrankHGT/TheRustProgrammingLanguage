use std::ops::Deref;

use smartpointer::{List::{Cons, Nil}, MyBox};

fn box_smartpointer() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn recursive_list() {
    let _list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    // &m -> m.defer() -> &String -> string.defer() -> &str
    hello(&m);
    // 1. *m -> *(m.defer()) -> String
    // 2. &(*m) -> &String
    // 3. &(*m)[..] -> &String[..] -> &str
    hello(&(*m)[..]);
}

fn reference_and_dereference() {
    normal_reference();

    box_reference();

    mybox_reference();
}

fn mybox_reference() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn box_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // you can deference Box<T> just like reference,
    // cause it impl Deref trait
    assert_eq!(5, *y);
}

fn normal_reference() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn main() {
    box_smartpointer();

    recursive_list();

    reference_and_dereference();

    deref_coercion();
}
