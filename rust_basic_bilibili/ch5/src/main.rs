fn main() {
    println!("Hello, world!");

    /*
           定義與實例化 Struct
    */
    // 基礎
    {
        // 如果其中一個 Field 是可變的, 則所有的都是可變的
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }

        let user = User {
            username: String::from("linonon"),
            email: String::from("qq"),
            sign_in_count: 556,
            active: true,
        };

        println!("user name: {}", user.username);
        println!("email: {}", user.email);
        println!("sign_in_count: {}", user.sign_in_count);
        println!("active: {}", user.active);
    }
    //
    {
        let w = 30;
        let l = 50;

        println!("{}", area(w, l));

        fn area(width: u32, length: u32) -> u32 {
            width * length
        }

        let rect = (30, 50);

        println!("{}", area_t(rect));
        fn area_t(dim: (u32, u32)) -> u32 {
            dim.0 * dim.1
        }

        // Like traid
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            length: u32,
        }

        let rect_s = Rectangle {
            width: 30,
            length: 50,
        };

        // std::fmt::Display
        println!("{}", area_s(&rect_s));

        // std::fmt::Debug
        println!("{:?}", rect_s);

        fn area_s(rect: &Rectangle) -> u32 {
            rect.width * rect.length
        }
    }

    // Struct 方法
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            length: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.length
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.length > other.length
            }

            // 關聯函數, 不是方法
            // 通常用於 構造函數
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    length: size,
                }
            }
        }

        let rect_s = Rectangle {
            width: 30,
            length: 50,
        };
        let rect_s2 = Rectangle {
            width: 10,
            length: 40,
        };
        let rect_s3 = Rectangle {
            width: 35,
            length: 55,
        };
        let s = Rectangle::square(10);

        println!("{}", rect_s.area());
        println!("can hold: {}", rect_s.can_hold(&rect_s2));
        println!("can hold: {}", rect_s.can_hold(&rect_s3));
        println!("square: {}", s.area());
    }
}
