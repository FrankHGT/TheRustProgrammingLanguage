fn main() {
    // declare with type
    let _v: Vec<i32> = Vec::new();

    // marco for init Vec
    let mut v = vec![1, 2, 3];

    // add element to Vec
    v.push(4);

    // print Vec
    println!("{:?}", v);

    read_element_from_vec();

    read_element_does_not_exist();

    update_vec_when_hold_unmutable_reference();

    iterate_vec();
} // <- out of scope, Vec and element in it will be dropped

fn read_element_from_vec() {
    let v = vec![1, 2, 3, 4, 5];

    // will panic in runtime when access &v[2]
    // let v: Vec<i32> = Vec::new();

    let third: &i32 = &v[2];

    println!("The third element is: {}", third);
    
    let v: Vec<i32> = Vec::new();

    match v.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("The vector doesn't have a third element"),
    }
}

fn read_element_does_not_exist() {
    let v = vec![1, 2, 3, 4, 5];

    // will panic in runtime when access &v[10]
    // let does_not_exist = &v[10];
    let does_not_exist = v.get(100);

    println!("The element does not exist: {:?}", does_not_exist);
}

fn update_vec_when_hold_unmutable_reference() {
    let mut v = vec![1, 2, 3, 4, 5];

    // unmutable reference
    let first = &v[0];

    // update action, mutable reference
    // compile error, cannot assign to data in `v` because it is borrowed
    // v.push(6);

    println!("The first element is: {}", first);
}

fn iterate_vec() {
    let mut v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
}

fn use_enum_store_multible_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(9.0),
    ];
}
