use colored::Colorize;
use regex::Regex;

pub fn __(s: &str) {
    let s = match s {
        "" => format!("// {:-^20}", " END "),
        _ => String::from("// - ") + s,
    };

    println!("\n{}", s.green());
}

pub fn pln1<T>(code: &str, result: T)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    let code = colorize_code(&code);
    println!("{} = {:?}", code, result);
}

pub fn comment(str: &str) {
    println!("{}", format!("//   - {}", str).green())
}

pub fn comment_lv2(str: &str) {
    println!("{}", format!("//     - {}", str).green())
}

#[test]
fn test__() {
    let cn = "数据体传入到什么地方";
    __(cn);
    __("数据在这里");
    __("abdcefrtg");
    __("abc");
    __("");
}

enum CodeType {
    Var,
    Func,
    String,
}

struct MyColor {
    r: u8,
    g: u8,
    b: u8,
}

const PURPLE: MyColor = MyColor {
    r: 210,
    g: 168,
    b: 255,
};
const BLUE: MyColor = MyColor {
    r: 121,
    g: 192,
    b: 255,
};

fn colorize_code(code: &str) -> String {
    let items: Vec<&str> = code.split('.').collect();
    println!("{:?}", items);
    let mut result = String::new();
    for (i, item) in items.iter().enumerate() {
        let r = if item.contains("(") {
            let left_barket_index = item.find('(').unwrap();
            let head = &item[..left_barket_index + 1].truecolor(PURPLE.r, PURPLE.g, PURPLE.b);
            let in_barket = &item[left_barket_index + 1..item.len() - 1];
            let in_barket = if in_barket.contains("\"") {
                let left_barket_index = in_barket.find('\"').unwrap();
                let head = &in_barket[..left_barket_index + 1].truecolor(BLUE.r, BLUE.g, BLUE.b);
                let in_barket = &in_barket[left_barket_index + 1..in_barket.len() - 1];

                format!(
                    "{}{}{}",
                    head,
                    in_barket.truecolor(BLUE.r, BLUE.g, BLUE.b),
                    "\"".truecolor(BLUE.r, BLUE.g, BLUE.b),
                )
            } else {
                in_barket.to_string()
            };

            format!(
                "{}{}{}",
                head,
                in_barket,
                ")".truecolor(PURPLE.r, PURPLE.g, PURPLE.b),
            )
        } else {
            item.to_string()
        };
        if i != 0 {
            result.push('.');
        }
        result.push_str(&r);
    }
    result
}

#[test]
fn test_regex() {
    let cases = vec!["", "abc", r#"x().y("").z.o("abc").expect(message)"#];

    for code in cases.iter() {
        println!("{}", colorize_code(code));
    }
}
