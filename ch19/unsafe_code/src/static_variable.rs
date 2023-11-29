

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn unmutable_static_variable() {
    // access unmutable static variable is safe
    println!("name is: {}", HELLO_WORLD);
}

fn add_to_count(inc: u32) {
    // access mutable static variable is unsafe
    unsafe {
        COUNTER += inc;
    }
}

fn mutable_static_variable() {
    add_to_count(3);

    // access mutable static variable is unsafe
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

pub fn run() {
    unmutable_static_variable();
    mutable_static_variable();
}