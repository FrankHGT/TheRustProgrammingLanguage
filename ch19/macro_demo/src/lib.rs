mod myvec;

create_function!(foo);
create_function!(bar);

pub fn run() {
    let nums = myvec![1, 2, 3];

    println!("nums from myvec!: {:?}", nums);

    say_hello!();

    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    })
}