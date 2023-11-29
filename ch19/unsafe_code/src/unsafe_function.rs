use std::slice;

unsafe fn dangerous() {

}

fn my_split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn wrap_unsafe_in_safe() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v;

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // after that, a, b is never used, so out of scope, other can borrow it
    let (a, b) = my_split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_ffi_from_c() {
    unsafe {
        println!("Absolute value of -3 according ot C: {}", abs(-3));
    }
}

pub fn run() {
    unsafe {
        dangerous();
    }
    wrap_unsafe_in_safe();
    call_ffi_from_c();
}