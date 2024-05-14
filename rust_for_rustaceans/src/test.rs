use std::mem;

#[test]
fn listing_1_6() {
    let x = 42;
    let mut y = &x;
    let z = &mut y;

    // let mut y = &x;
    // *z += 1; // binary assignment operation `+=` cannot be applied to type `&{integer}`

    // let mut y = x;
    // *z += 1; // ok

    // x += 1; // cannot mutate immutable variable `x`
    // y += 1; // cannot use `+=` on type `&{integer}`
    // z += 1; // cannot use `+=` on type `&mut &{integer}`

    // println!("x: {}, y: {},  z: {}", x, y, z) // cannot borrow `y` as immutable because it is also borrowed as mutable
    println!("x: {}, z: {}", x, z) // ok
}

#[test]
fn listing_1_7() {
    fn replace_with_84(s: &mut Box<i32>) {
        // let was = *s; // cannot move out of `*s` which is behind a mutable reference
        let was = std::mem::take(s); // s -> 0; was -> 42

        *s = was; // s -> 42

        let mut r = Box::new(84);
        std::mem::swap(s, &mut r);
        assert_ne!(*r, 84);
    }

    let mut s = Box::new(42);
    replace_with_84(&mut s);
}

#[test]
fn listing_1_8() {
    let mut x = Box::new(42);
    let r = &x;
    let rand = rand::prelude::random::<f64>();
    println!("rand: {}", rand);

    if rand > 0.5 {
        *x = 84;
        println!("x: {}", x);
    } else {
        println!("r: {}", r);
    }

    println!("x: {}", x);
    // println!("r: {}", r); // cannot assign to `*x` because it is borrowed
}

#[test]
fn listing_1_9() {
    let mut x = Box::new(42);
    let mut z = &x;

    for i in 0..100 {
        println!("z: {}", z);
        x = Box::new(i);
        z = &x; // cannot comment out, otherwise `z` will be dangling
    }

    println!("final x: {}", x);
    println!("final z: {}", z);
}

#[test]
fn listing_1_10() {
    struct StrSplit<'s, 'p> {
        delimiter: &'p str, // if use ['s], the &c.to_string() will be error
        document: &'s str,
    }

    impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
        type Item = &'s str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.document.is_empty() {
                return None;
            }

            let next_delim = self.document.find(self.delimiter)?;
            let until_delimiter = &self.document[..next_delim];
            self.document = &self.document[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        }
    }

    fn str_before(s: &str, c: char) -> Option<&str> {
        StrSplit {
            delimiter: &c.to_string(),
            document: s,
        }
        .next()
    }

    let s = "Hello, world!";
    println!(
        "(s before '!') before ',': {:?}",
        str_before(str_before(s, '!').unwrap(), ',').unwrap()
    );
}

#[test]
fn listing_1_11() {
    struct MutStr<'a, 'b> {
        s: &'a mut &'b str,
    }
    let mut s = "hello";

    // *MutStr { s: &mut s }.s = "world"; // equal to below
    let x = MutStr { s: &mut s };
    *x.s = "world";

    println!("s: {}", s);
}

#[test]
fn listing_2_1() {
    #[repr(C)]
    #[allow(dead_code)]
    struct Foo {
        tiny: bool,  // 1 bit -> 1 byte
        normal: u32, // 32 bits -> 4 bytes
        small: u8,   // 8 bits -> 1 byte
        long: u64,   // 64 bits -> 8 bytes
        short: u16,  // 16 bits -> 2 bytes
    }

    // 32位系统, 以 4 byte 对齐, 則布局会变成
    // |  tiny + padding | normal | small + padding |  long  |  short |       padding      |
    // | 1 byte + 3 byte | 4 byte | 1 byte + 3 byte | 8 byte | 2 byte | = 24 byte + 8 byte | = 32 byte
}

#[test]
fn listing_2_1_1() {
    struct A {
        x: i32,
    }

    struct NewA(A);

    #[repr(transparent)]
    struct TransparentNewA(A);

    #[repr(C)]
    struct CA(A);

    println!("Size of A: {}", mem::size_of::<A>());
    println!("Size of NewA: {}", mem::size_of::<NewA>());
    println!(
        "Size of TransparentNewA: {}",
        mem::size_of::<TransparentNewA>()
    );
    println!("Size of CA: {}", mem::size_of::<CA>());
}
