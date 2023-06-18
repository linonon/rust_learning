fn main() {
    println!("Hello, world!");

    // Module
    // 模块系统:
    // - Pakcage(单元): Cargo 的特性, 让你构建测试共享 crate
    // - Crate(单元包): 一个模块树, 可产生一个 library 或可执行文件
    // - Module(模块)=> use:
    // - Path(路径): 为了struct, function 或 module 等项命名的方式
}

// fn server_order() {}

mod back_of_house {
    // fn fix_incorrect_order() {
    //     cook_order();
    //     super::server_order();
    // }

    // fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: (String::from(toast)),
                seasonal_fruit: (String::from("peaches")),
            }
        }
        pub fn get_seasonal_fruit(self) -> String {
            self.seasonal_fruit
        }
    }
}

pub fn eat_at_resturant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("seasonal_fruit: {}", meal.get_seasonal_fruit());
    // meal.seasonal_fruit = String::from("blueberries");
}

// use std::collections::*;
// use std::{
//     fmt::Result,
//     io::{self, Result as IoResult},
// };
// fn f1() -> fmt::Result {}
// fn f2() -> io::Result {}
