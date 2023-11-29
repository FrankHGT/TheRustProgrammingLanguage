fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

pub fn run() {
    let point = (3, 5);

    print_coordinates(&point);
}