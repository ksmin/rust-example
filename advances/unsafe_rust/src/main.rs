use std::slice;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    unsafe {
        dangerous(&num as *const i32, &mut num as *mut i32);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {(
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
    )}
}

unsafe fn dangerous(r1: *const i32, r2: *mut i32) {
    println!("r1 is : {}", *r1);
    println!("r2 is : {}", *r2);
}