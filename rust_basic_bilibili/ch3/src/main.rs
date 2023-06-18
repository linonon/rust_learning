fn main() {
    println!("Hello, world!");

    variable();

    constant();

    shadowing();

    biao_zhu_lei_xing();

    biao_liang_lei_xing();

    fu_he_lei_xing();

    han_shu_lei_xing(1, "2".to_string());

    let x = add_five(2);
    println!("x: {}", x);

    if_else();
    if_esle2();

    xun_huan();
}

fn variable() {
    // let x = 5;
    // println!("x: {}", x);

    // x = 6;
    // println!("x: {}", x);

    let mut y = 5;
    println!("y: {}", y);

    y = 6;
    println!("y: {}", y);
}

// const
fn constant() {
    // naming rules: all capitalize
    const MAX_POINT: u32 = 100_000;
    println!("max point: {}", MAX_POINT)
}

// 隱藏
fn shadowing() {
    let x = 5;
    // 直接改變數值則會出錯
    // x = 6;

    // 這個 x 代表是的是一個新的變量 x
    let x = x + 1;
    println!("x: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("now, spaces: {}", spaces);
}

fn biao_zhu_lei_xing() {
    let guess: u32 = "42".parse().expect("cannot parse");
    // ↓ 出錯
    // let guess = "42".parse().expect("cannot parse");
    println!("guess: {}", guess);
}

fn biao_liang_lei_xing() {
    // u[8-128], i[8-128]...
    // arch i[size] u[size], depend on architecture.

    // 類型後綴
    let num = 57u8;
    println!("num : {}", num);

    // f32, f64(defalut)
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x, y: {}, {}", x, y);

    // bool
    let t = true;
    let f: bool = false;
    println!("t, f: {}, {}", t, f);

    // char: 'a' => Unicode
    let x = 'z';
    let y: char = '中';
    println!("x, y: {}, {}", x, y)
}

fn fu_he_lei_xing() {
    // Tuple: 長度固定
    let tuple: (i32, f32, u8) = (500, 6.3, 1);
    let (x, y, z) = tuple;
    println!("x, y, z:y {}, {}, {}", x, y, z);
    println!("x, y, z: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    // Array
    let arr = [1, 2, 3, 4, 5];
    let arrs: [i32; 5] = [1, 2, 3, 2, 1];
    let arrs2 = [3; 10];
    println!("arr, arrs, arrs2: {}, {}, {}", arr[0], arrs[0], arrs2[0])
}

fn han_shu_lei_xing(x: i32, str: String) {
    println!("functional class");
    println!("x, str: {}, {}", x, str);
}

fn add_five(x: i32) -> i32 {
    x * 3
}

fn if_else() {
    let num = 6;
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4,3,2")
    }
}

fn if_esle2() {
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("number is: {}", number);
}

fn xun_huan() {
    let mut count = 0;
    loop {
        println!("again");
        count += 1;
        if count == 3 {
            break;
        }
    }

    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    for element in a.iter() {
        println!("element: {}", element);
    }

    // 從 1 到 3
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!")
}
