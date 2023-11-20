// generic function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // can't compile for now
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_function() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largset number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// generic struct
struct Point<T> {
    x: T,
    y: T,
}

// generic for method
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// generic specialization for f32 type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic_struct() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // x and y must be same type
    // let wont_work = Point { x: 5, y: 4.0 };
    let _some_point = Point2 { x: 5, y: 5.0 };
}

fn generic_method() {
    let p = Point {x: 5, y: 10 };

    println!("p.x = {}", p.x());


    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn main() {
    generic_function();
    generic_struct();
    generic_method();
}
