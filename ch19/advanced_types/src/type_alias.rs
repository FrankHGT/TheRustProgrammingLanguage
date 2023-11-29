fn type_alias_and_origin_type_are_same_type() {
    // type alias is just same type with origin type
    // not like newtype Kilometers(i32)
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

}

fn use_type_alias_reduce_code_duplication() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(_f: Thunk) {

    }

    fn _returns_long_type() -> Thunk {
        Box::new(|| ())
    }

    // see this trait for more
    // std::io::Write
}

pub fn run() {
    type_alias_and_origin_type_are_same_type();
    use_type_alias_reduce_code_duplication();
}