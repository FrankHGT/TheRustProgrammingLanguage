pub fn run() {
    let mut num = 5;
    
    // create raw pointer
    let r1 = &num as *const i32;
    // must safe, cause num is valid, &mut has valid address
    let r2 = &mut num as *mut i32;

    // may not able to access 0x012345, may cause segmentation fault
    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        // dereference a raw pointer need in unsafe block
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}