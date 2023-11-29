use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Rhs will be Point by default
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

// explicit assign generic parameters
// or Rhs will be Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

fn default_generic_parameters_and_operator_overloading() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 0, y: 1 },
        Point { x: 1, y: 1 });

    assert_eq!(Millimeters(1) + Meters(1), Millimeters(1001));
}

pub fn run() {
    default_generic_parameters_and_operator_overloading();
}