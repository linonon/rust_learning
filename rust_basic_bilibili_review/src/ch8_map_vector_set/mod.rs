#[allow(unused)]
use crate::pkg::*;

#[test]
fn vec() {
    __("创建 Vector");
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    println!("v1: {:?}, v2: {:?}", v1, v2);

    __("可变 Vector");
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    v3.push(4);
    println!("v3: {:?}", v3);

    __("读取 Vector");
    let v4 = vec![1, 2, 3, 4, 5];
    let thrid: &i32 = &v4[2];
    println!("&v4[2] = {:?}", thrid);

    match v4.get(2) {
        Some(val) => println!("v4.get(2) = {:?}", val),
        None => println!("element not found"),
    }

    __("使用 enum 来存储多种数据类型");
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(0),
        SpreadSheetCell::Float(1.1),
        SpreadSheetCell::Text("this row".to_string()),
    ];
    println!("spread sheet cell in vector:\n{:?}", row);

    __("");
}

#[test]
fn string() {
    __("更新 String");
    let mut s = String::from("foo");
    s.push_str(" bar");
    pln1("s.push_str(\" bar\")", &s);

    let mut s1 = String::from(" new bar");
    s.push_str(&s1);
    pln1("s.push_str(&s1)", &s);
    s1.push_str(" foo");
    println!("s1.push_str(\" foo\")");
    print!("=> ");
    pln1("s1.push_str(&s1)", &s);

    __("字符串长度");
    let len = String::from("Hola").len();
    pln1("'hola'.len()", &format!("{}", len));

    __("");
}

#[test]
fn hash_map() {
    use std::collections::HashMap;

    __("创建 Hash map");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("k".to_string(), 0);
    println!("score = {:?}", scores);

    __("使用 collect 方法创建 HashMap");
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![50, 100];

    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(intial_scores.into_iter()).collect();
    println!("scores: HashMap<_,_> = {:?}", scores);
    println!("rust 会自动推导 HashMap 里面的类型");

    __("HashMap 和所有权");
    comment("对于实现了 Copy trait 的类型 , 值会复制到 HashMap 中");
    comment("对于拥有所有权的值(如 String), 值会被移动");
    comment("如果插入引用, 在 HashMap 有效的期间 , 被引用的值必须保持有效");

    __("访问 HashMap 中的值");
    let score = match scores.get(&"Blue".to_string()) {
        Some(s) => s,
        None => return,
    };

    trait ToString {
        fn to_string(self) -> String;
    }

    impl ToString for i32 {
        fn to_string(self) -> String {
            format!("{}", self)
        }
    }
    pln1("scores.get('Blue') = ", &score.to_string());

    __("遍历 HashMap");
    for (i, (k, v)) in scores.iter().enumerate() {
        println!("{} = {}: {}", i, k, v)
    }

    __("entry(): 当 key 不存在才插入");
    scores.entry(String::from("Black")).or_insert(99);

    for (i, (k, v)) in scores.iter().enumerate() {
        println!("{} = {}: {}", i, k, v)
    }

    __("修改 HashMap 里的值");
    let text = "hello world good world";
    let mut map = HashMap::new();

    for word in text.split(" ") {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    comment("*count += 1 : 解引用没问题, 只要不发生所有权转移就好");
    println!("{:#?}", map);

    __("");
}
