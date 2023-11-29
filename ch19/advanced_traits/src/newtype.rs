use std::fmt;

struct Wrapper(Vec<String>);

// you can't impl Display trait for Vec<String>,
// cause neither Display trait or Vec<String> is in your mod
// this called orphan rule
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}