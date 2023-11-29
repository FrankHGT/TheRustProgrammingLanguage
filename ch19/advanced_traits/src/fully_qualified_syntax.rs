trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn run() {
    let person = Human;

    // will default call struct's method when have multiple choices
    // this is work cause these method can receive self type, so rust
    // compiler can tell which method to call
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    // this is ok, like we call static method in C++
    println!("A baby dog is called a {}", Dog::baby_name());
    // this is not ok, cause many type can implement Animal trait
    // without self type, rust compiler can't determine which method to call
    // println!("A baby dog is called a {}", Animal::baby_name());

    // that's when we need fully qualified syntax!, use this syntax you
    // can finally call Animal's baby_name with Dog type
    //                                    <Type as Trait>
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}