use std::rc::Rc;

use smartpointer::{List::{Cons, Nil}, MyBox, CustomSmartPointer, RcList};

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

fn drop_trait() {
    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };

    println!("CustomSmartPointer created.");
}

fn drop_early() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn rc_smartpointer() {
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("rc after creating a = {}", Rc::strong_count(&a));
    let _b = RcList::Cons(3, Rc::clone(&a));
    println!("rc after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcList::Cons(4, Rc::clone(&a));
        println!("rc after creating c = {}", Rc::strong_count(&a));
    }
    println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
}

fn main() {
    box_smartpointer();

    recursive_list();

    reference_and_dereference();

    deref_coercion();

    drop_trait();

    drop_early();

    rc_smartpointer();
}
