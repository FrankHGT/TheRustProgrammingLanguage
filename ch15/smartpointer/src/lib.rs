mod limit_tracker;

// recursive type, size will never computed
// pub enum List {
//     Cons(i32, List),
//     Nil,
// }

// use box indirect the relation
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

use std::{ops::Deref, rc::Rc};

// tuple struct, use struct as a tuple
// it's behavior like tuple, with a struct name
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}