fn generic<T: ?Sized>(_t: &T) {
    println!("{}", std::any::type_name::<T>())
}

trait Test {
    
}

struct TestStruct {}

impl Test for TestStruct {}

pub fn run() {
    generic("x");
    let test = TestStruct {};
    generic(&test);
}