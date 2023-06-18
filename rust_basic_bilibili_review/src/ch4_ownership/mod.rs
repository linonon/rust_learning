/// # Test
///
///
///
#[test]
fn ch3() {
    scope();
    type_string();
    rust_move();
    copy_trait();
    borrow();
    borrow_mut();
    slice();
}

/// ## 作用域
#[allow(dead_code)]
fn scope() {}

/// ## String 類型
/// - 會在 heap 上分配存儲空間
#[allow(dead_code)]
fn type_string() {
    // "hello" 存在 stack 上, from() 将申请内存空间(heap), 以支持可变性
    let mut s = String::from("hello");
    // s 存在 heap 上 , 所以可以改变
    s.push_str("ass");
    println!("{s}")
}

#[allow(dead_code)]
fn rust_move() {
    // x,y 都是被压到 stack 中
    let mut x = 5;
    let mut y = x;
    x += 1;
    y += 2;

    // 因为 5 是基础数据类型
    println!("{x},{y}");

    // String 版
    let a = String::from("A");
    let mut b = a;

    // a.push_str("test a");
    // ↑ err:
    // 		borrow of moved value: `a`
    // 		value borrowed here after move
    // reason:
    //		a 的所有权 move 到了 b 上.
    b.push_str("test b");
}

/// ## Copy trait
/// - 可用于像整数这样完全存放在 stack 上面的类型
/// - 如果一个类型实现了 copy trait, name 旧的变量在赋值后仍可以使用
/// - 如果一个类型或者该类型一部分实现了`Drop trait`, 那么 Rust 不允许它再去实现 Copy
/// Trait 了
#[allow(dead_code)]
fn copy_trait() {
    fn take_ownership(s: String) {
        println!("s: {s}")
    }
    fn makes_copy(i: i32) {
        println!("i: {i}")
    }

    let s = String::from("str");
    take_ownership(s);
    // println!("{s}");
    // ↑ err:
    //      value moved.
    //      move occurs because `s` has type `String`,
    //      which does not implement the `Copy` trait.

    let i = 5;
    makes_copy(i);
    println!("{}", i + 1);
}

#[allow(dead_code)]
fn borrow() {
    fn calculate_length(s1: &String) -> usize {
        s1.len()
    }
    let s1 = String::from("Hello");
    let len: usize = calculate_length(&s1);
    println!("'{s1}' length is {len}");
}

#[allow(dead_code)]
fn borrow_mut() {
    fn calculate_length(s1: &mut String) -> usize {
        s1.push_str(", world");
        s1.len()
    }
    let mut s1 = String::from("Hello");
    let len: usize = calculate_length(&mut s1);
    println!("'{s1}' length is {len}");
}

#[allow(dead_code)]
fn slice() {
    let mut s = String::from("hello world");
    let word_index = first_word(&s);
    // s.clear();
    // println!("word index: {word_index}");
    println!("word: {word_index}");

    s.push_str(" strinag");
    println!("s: {s}");

    // fn first_word(s: &String) -> usize {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }
}
