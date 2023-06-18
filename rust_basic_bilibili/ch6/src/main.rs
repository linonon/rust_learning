fn main() {
    println!("Hello, world!");

    /*
           6 枚舉
    */

    // 枚舉值
    {
        #[derive(Debug)]
        enum IPAddKind {
            V4,
            V6,
        }

        let ipv4 = IPAddKind::V4;
        let ipv6 = IPAddKind::V6;

        route(ipv4);
        route(ipv6);

        fn route(ip: IPAddKind) {
            println!("route:{:?}", ip);
        }
    }
    {
        #[derive(Debug)]
        enum IPAddKind {
            V4,
            V6,
        }

        #[derive(Debug)]
        struct IPAddr {
            kind: IPAddKind,
            address: String,
        }

        let home = IPAddr {
            kind: IPAddKind::V4,
            address: String::from("127.0.0.1"),
        };
        println!("{:?}, {}", home.kind, home.address);

        let loopback = IPAddr {
            kind: IPAddKind::V6,
            address: String::from("::1"),
        };
        println!("{:?}", loopback);

        #[derive(Debug)]
        enum IPAddrKindV2 {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home_v2 = IPAddrKindV2::V4(127, 0, 0, 1);
        let lookback_v2 = IPAddrKindV2::V6(String::from("::1"));
        println!("{:?}", home_v2);
        println!("{:?}", lookback_v2);
    }
    // Message
    {
        #[derive(Debug, PartialEq)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                match self {
                    Message::Move { x, y } => {
                        println!("call self, but is Move: x = {}, y = {}", x, y)
                    }
                    _ => println!("call self: {:?}", self),
                }
            }
        }

        let msg_move = Message::Move { x: 10, y: 12 };
        let msg_quit = Message::Quit;
        let msg_write = Message::Write(String::from("write"));
        let msg_change_color = Message::ChangeColor(10, 10, 10);

        println!("{:?}", msg_move);
        println!("{:?}", msg_quit);
        println!("{:?}", msg_write);
        println!("{:?}", msg_change_color);

        msg_move.call();
        msg_quit.call();
        msg_write.call();
        msg_change_color.call();
    }
    // Option
    {
        // Rust 沒有 Null
        // Rust: Option<T>
        // enum Option<T> {
        //     Some(T),
        //     None,
        // }

        let num = Some(5);
        let string = Some("A String");

        let absent_number: Option<i32> = None;
        let normal_number: i32 = 1;

        println!(
            "{:?},{:?},{:?},{}",
            num, string, absent_number, normal_number
        );
        // 无法转换
        // let tmp = absent_number + normal_number;
    }

    // match
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("penny");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter form {:?}!", state);
                    25
                }
            }
        }

        let d = Coin::Penny;
        let e = Coin::Nickel;
        let f = Coin::Dime;
        let g1 = Coin::Quarter(UsState::Alaska);
        let g2 = Coin::Quarter(UsState::Alabama);

        println!("value: {:?}", value_in_cents(f));
        println!("value: {:?}", value_in_cents(d));
        println!("value: {:?}", value_in_cents(e));
        println!("value: {:?}", value_in_cents(g1));
        println!("value: {:?}", value_in_cents(g2));
    }

    // match: 匹配 Option<T>
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        // fn plus_one(x: Option<i32>) -> Option<i32> {
        // match 匹配需要全部处理, 不可以少
        //     match x {
        //         None => None,
        //         Some(i) => Some(i + 1),
        //     }
        // }
        fn plus_one(x: Option<i32>) -> Option<i32> {
            x.map(|i| i + 1)
        }

        println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    }
    // match 通配符
    // _ : 通配符
    {
        let v = 0u8;
        match v {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => println!("{}", v),
        }
    }

    // if let
    // 处理只关心一种匹配而忽略其他匹配的情况
    // less typing, less indentation, less boilerplate code.
    {
        let v = Some(0u8);

        if let Some(3) = v {
            println!("three")
        } else {
            println!("v: {:?}", v)
        }

        // why we use `if let` not `if ==`
        // if Some(3) == v {
        //     println!("three")
        // } else {
        //     println!("v: {:?}", v)
        // }
        #[derive(Debug)]
        enum Choices {
            A,
            B,
            C(i32),
        }

        let choices: Choices = Choices::C(2);
        if let Choices::C(value) = choices {
            println!("{}", value * 2)
        }

        let choices: Choices = Choices::A;
        if let Choices::A = choices {
            println!("choices enum = {:?}", choices)
        }

        let choices: Choices = Choices::B;
        if let Choices::B = choices {
            println!("choices enum = {:?}", choices)
        }
    }
}
