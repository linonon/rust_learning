#[allow(unused)]
use crate::pkg::*;

#[test]
fn enumerate() {
    __("创建枚举类型");
    #[derive(Debug)]
    enum IpAddKind {
        V4,
        V6,
    }

    let ipv4 = IpAddKind::V4;
    let ipv6 = IpAddKind::V6;

    println!("{:#?}, {:#?}", ipv4, ipv6);

    __("数据附加到结构体中");
    #[allow(dead_code)]
    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }

    __("数据附加到 enum 中");
    #[derive(Debug)]
    enum IpAddrEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("home:{:?}, loopback:{:?}", home, loopback);

    __("为 enum 定义方法");
    #[derive(Debug)]
    enum Message {
        Move { x: i32, y: i32 },
        Write(String),
        RGB(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Move { x, y } => println!("move: x: {} y: {}", x, y),
                Message::Write(s) => println!("write: {}", s),
                Message::RGB(r, g, b) => println!("R: {} G: {} B: {}", r, g, b),
            }
        }
    }

    let m = Message::Move { x: 10, y: 2 };
    let w = Message::Write("writing".to_string());
    let rgb = Message::RGB(10, 200, 30);
    m.call();
    w.call();
    rgb.call();
}

#[test]
fn enum_option() {
    __("Option<T> 明确表明变量是否会有 Null 值");
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // if x == y {}
    // ↑ err:
    // 		mismatched types
    // 		expected type `i8`
    // 		found enum `Option<i8>`
    let y = match y {
        Some(y) => y,
        None => 0,
    };
    if x == y {
        println!("x == y");
    } else {
        println!("x = {}, y = {}", x, y);
    };
}

#[test]
fn match_learning() {
    __("match 表达式");
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    impl Coin {
        fn value(&self) -> u8 {
            match self {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;

    println!(
        "penny: {}\nnickel: {}\ndime: {}\nquarter: {}\n",
        p.value(),
        n.value(),
        d.value(),
        q.value()
    );

    __("练习 Add trait 使用");
    impl std::ops::Add for &Coin {
        type Output = u8;

        fn add(self, rhs: &Coin) -> Self::Output {
            &self.value() + &rhs.value()
        }
    }
    let my = &p + &n;
    println!("p: {}, n: {}, p+n = {}", p.value(), n.value(), my);

    __("enum 嵌套 enum, 绑定值");
    #[derive(Debug)]
    enum UsState {
        NewYork,
    }
    enum CoinV2 {
        Quarter(UsState),
    }
    impl CoinV2 {
        fn value(&self) -> u8 {
            match self {
                CoinV2::Quarter(state) => {
                    println!("State quarter from {:?}", state);
                    25
                }
            }
        }
    }
    let c = CoinV2::Quarter(UsState::NewYork);
    println!("{}", c.value());

    __("匹配 Option<T>");
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!(
        "six: {}, none: {}",
        if let Some(i) = six { i } else { 0 },
        if let None = none { 0 } else { 0 },
    )
}
