fn main() {
    println!("Hello, ch18");
}

/// ## Match
/// 需要窮舉 , 否則需要用`_`作默認值
/// ## if let
/// 不會窮舉
#[test]
fn class_18_1_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("favorite color: {}", color);
    } else if is_tuesday {
        println!("is tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("age out of range: {}", age);
        } else {
            println!("age below 30: {}", age);
        }
    } else {
        println!("no condition true")
    }
}

#[test]
fn class_18_1_while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(val) = stack.pop() {
        println!("{}", val);
    }
}

#[test]
fn class_18_1_for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index: {}", value, index);
    }
}

#[test]
fn c18_3() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='f' => println!("early ASCII letter"),
        'i'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

#[test]
fn c18_3_dispatch() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };

    let Point { x: a, y: b } = p;
    assert_eq!(a, 1);
    assert_eq!(b, 2);

    match p {
        Point { x: 1, y: b } => println!("y = {}", b),
        Point { x: a, y: 2 } => println!("x = {}", a),
        Point { x: a, .. } => println!("just x: {}", a),
    }
}
