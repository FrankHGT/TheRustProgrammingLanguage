use std::fmt::Display;

struct Part<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> self {
        Self {
            x,
            y,
        }
    }
}

// specialization for type who impl Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        let mut larger = self.x;
        let mut larger_v = String::from("x");
        if self.x <= self.y {
            larger = self.y;
            larger_v = String::from("y");
        }

        println!("The larger member is {} = {}", larger_v, larger);
    }
}

// blanket implementations
impl<T: Display> ToString for T {

}