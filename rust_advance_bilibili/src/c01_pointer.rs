#[cfg(test)]
use std::mem::size_of;

#[cfg(test)]
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
#[cfg(test)]
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

#[test]
fn what_is_pointer() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b:{:p}, c:{:p}", a, b, c);
}

#[test]
fn what_is_pointer_v2() {
    let a: usize = 42;
    let b: Box<[u8]> = Box::new(B);
    let c: &[u8; 11] = &C;

    println!("a (unsigned 整数):");
    println!("  地址:   {:p}", &a);
    println!("  大小:   {:?} bytes", size_of::<usize>());
    println!("  值:     {:?}\n", a);

    println!("b (B 在 Box 里):");
    println!("  地址:   {:p}", &b);
    println!("  大小:   {:?} bytes", size_of::<Box<[u8]>>());
    println!("  值:     {:?}\n", b);

    println!("c (C 的引用):");
    println!("  地址:   {:p}", &c);
    println!("  大小:   {:?} bytes", size_of::<&[u8; 11]>());
    println!("  值:     {:?}\n", c);

    println!("B (10 bytes 的数组):");
    println!("  地址:   {:p}", &B);
    println!("  大小:   {:?} bytes", size_of::<&[u8; 10]>());
    println!("  值:     {:?}\n", B);

    println!("C (11 bytes 的数组):");
    println!("  地址:   {:p}", &C);
    println!("  大小:   {:?} bytes", size_of::<&[u8; 11]>());
    println!("  值:     {:?}\n", C);
}

#[cfg(test)]
use std::{borrow::Cow, ffi::CStr, os::raw::c_char};

#[test]
fn what_is_pointer_v3() {
    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;
        // c = CStr::from_ptr(c_ptr).to_string_lossy();
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}
