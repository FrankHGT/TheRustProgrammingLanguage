use std::{rc::Rc, cell::RefCell};

mod circular_reference;
use smartpointer::{List::{Cons, Nil}, MyBox, CustomSmartPointer, RcList, MutableList};

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

fn ref_alongside_refmut() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(MutableList::Cons(Rc::clone(&value), Rc::new(MutableList::Nil)));

    let b = MutableList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = MutableList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // explain:
    // 1. borrow_mut's function signature indicate it's need a &self
    // 2. Rust compile will automaiclly insert * operator to dereference it,
    // ie: (*value).borrow_mut(), 
    // 3. cause Rc<T> impl Deref trait, it will turn to RefCell<T>.borrow_mut() (just for example)
    // this function called automatic reference and dereference(mentioned in ch5)
    // 4. borrow_mut() will return a RefMut<T>, *(value.borrow_mut()) will turn to &mut i32
    *value.borrow_mut() += 10;
    // 1. unpack b.next from &Rc<MutableList> => &MutableList
    // 2. &Rc<RefCell<i32>> => &mut i32
    match &b {
        MutableList::Cons(_, next) => match next.as_ref() {
            MutableList::Cons(v, _n) => *v.borrow_mut() += 10,
            MutableList::Nil => panic!("a is Nil"),
        },
        MutableList::Nil => panic!("b is Nil"),
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn circular_reference() {
    use circular_reference::List::Cons as CCons;
    use circular_reference::List::Nil as CNil;

    let a = Rc::new(CCons(5, RefCell::new(Rc::new(CNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // will trigger stack overflow, cause Debug trait
    // will recursivly print next item!
    // println!("a next item = {:?}", a.tail());
}

fn main() {
    box_smartpointer();

    recursive_list();

    reference_and_dereference();

    deref_coercion();

    drop_trait();

    drop_early();

    rc_smartpointer();

    ref_alongside_refmut();

    circular_reference();
}
