#[allow(unused)]
use crate::pkg::*;

#[test]
fn generic() {
    __("泛型");
    comment("泛型可以说是一种模板, 里面有一些'占位符'");
    comment("编译器在编译时, 才会将'占位符'替换为具体的类型");

    __("函数定义中的泛型");
    // fn largest<T>(list: &[T]) -> T {
    //     let mut largest = list[0];
    //     for &item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }

    __("结构体里的泛型");
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 3.0, y: 4.0 };
    println!(
        "integer: {{{},{}}}, float: {{{},{}}}",
        integer.x, integer.y, float.x, float.y
    );

    #[derive(Debug)]
    struct PointV2<T, U> {
        x: T,
        y: U,
    }
    let num = PointV2 { x: 1, y: 2.0 };
    println!("num: {{{},{}}}", num.x, num.y);

    __("为泛型数据结构实现方法");
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    pln1("integer.x()", integer.x());

    __("struct 里的泛型类型参数可以和方法的泛型类型参数不同");
    impl<T, U> PointV2<T, U> {
        #[allow(unused)]
        fn mixup<V, W>(self, other: PointV2<V, W>) -> PointV2<T, W> {
            PointV2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = PointV2 { x: 1, y: 2.0 };
    let p2 = PointV2 { x: 3.0, y: 'o' };
    let p3 = p1.mixup(p2);
    pln1("p1.mixup(p2)", format!("{:?}", p3));

    __("");
}

#[test]
fn trait_cp1() {
    __("trait");
    pub trait Summary {
        // 默认
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            String::from("(Read more... but new funciton)")
        }
    }

    __("使用 Trait Bount 有条件的实现方法");
    comment("在使用泛型类型参数的 impl 块使用 Trait bound, ");
    comment("我们可以有条件的为了实现特定 Trait 的类型来实现方法");
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Trait bound, 有条件的为实现了特定 Trait 的类型来实现方法
    impl<T: std::fmt::Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("Larger: x = {}", self.x)
            } else if self.x == self.y {
                println!("x:{} == y:{} ", self.x, self.y)
            } else {
                println!("Larger: y = {}", self.y)
            }
        }
    }

    __("");
}

#[test]
fn life_time() {
    __("生命周期例子");
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest is: {}", result);

    __("生命周期标注 - 例子");
    __("struct 标注生命周期");
    struct Structure<'a> {
        #[allow(dead_code)]
        part: &'a str,
    }

    let novel = String::from("call me linonon");
    let word = novel.split(' ').next().expect("Could not found a '.'");

    let _ = Structure { part: word };

    __("方法定义中的生命周期标注");
    comment("struct 字段的生命周期名");
    comment_lv2("在 impl 后`声明`");
    comment_lv2("在 struct 后`使用`");
    comment_lv2("这些声明周期是 struct 类型的一部分");
    let (.., x, y) = (0, 1, ..);

    println!("{:?},{:?}", x, y);
    println!("{:?}", b"066");
    println!("{}", b"066"[y][x]);

    __("");
}
