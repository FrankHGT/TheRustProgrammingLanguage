use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, next) => Some(next),
            List::Nil => None,
        }
    }
}