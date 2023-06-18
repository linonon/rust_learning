#[test]
fn ch5_struct() {
    struct_basic_user();
    learn_sturct_with_rectangle();
}

#[allow(dead_code)]
fn struct_basic_user() {
    #[derive(Clone, Debug)]
    struct User {
        user_name: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user_1 = User {
        user_name: String::from("linonon"),
        email: String::from("a@gmail.com"),
        sign_in_count: 10,
        active: true,
    };
    println!("{:?}", user_1);

    // --------------------------
    fn easy_build(user_name: String, email: String) -> User {
        return User {
            user_name,
            email,
            sign_in_count: 10,
            active: true,
        };
    }
    let easy_build_user = easy_build("linonon".to_string(), "126@123.com".to_string());
    println!("easy build user: {:?}", easy_build_user);

    // --------------------------
    fn update_struct(prev_user: User) -> User {
        return User {
            active: !prev_user.active,
            ..prev_user
        };
    }
    let update_user = update_struct(easy_build_user);
    println!("update user: {:?}", update_user);

    // --------------------------
    #[derive(Debug)]
    struct TupleStruct(usize, usize, usize);
    let tp_struct = TupleStruct(1, 1, 1);
    println!("tp struct: {:?}", tp_struct);
}

#[allow(dead_code)]
fn learn_sturct_with_rectangle() {
    let rect = (30, 50);
    fn area(dim: (u32, u32)) -> u32 {
        dim.0 * dim.1
    }
    println!("area = {}", area(rect));

    // --------------------------
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        length: u32,
    }
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    fn area_rect(rect: &Rectangle) -> u32 {
        rect.width * rect.length
    }
    println!("area rect: {}", area_rect(&rect));
    println!("{:?}", rect);

    // --------------------------
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.length
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.length > other.length
        }

        // --------------------------
        // 关联函数
        fn new(width: u32, length: u32) -> Rectangle {
            Self { width, length }
        }
        fn square(size: u32) -> Rectangle {
            Rectangle::new(size, size)
        }
    }
    println!("rect.area(): {}", rect.area());
    println!(
        "rect(30,50) can hold other(20,40): {}",
        rect.can_hold(&Rectangle::new(20, 40))
    );
    println!(
        "rect(30,50) can hold other(40,40): {}",
        rect.can_hold(&Rectangle::new(40, 40))
    );
    println!(
        "rect(30,50) can hold square(20): {}",
        rect.can_hold(&Rectangle::square(20))
    );
}
