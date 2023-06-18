use std::collections::HashMap;

fn main() {
    println!("Hello, collections!");

    {
        // let v: Vec<i32> = Vec::new();
        let mut v = vec![];
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);

        println!("{:?}", v);

        let third = &v[2];
        println!("Third is: {}", third);

        match v.get(100) {
            Some(x) => println!("{}", x),
            None => println!("None"),
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        let _first = &v[0];
        v.push(6);
        println!("v is: {:?}", v)
    }
    // String cp.1
    {
        let data = "initial contents";

        let s = data.to_string();
        let s1 = "initial contents".to_string();
        let s2 = String::from("initial contents");
        println!("s, s1, s2: {}, {}, {}", s, s1, s2);
    }
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{:?}", s);
        s.push('l');
        println!("{:?}", s);
    }
    {
        let s1 = String::from("foo");
        let s2 = String::from("bar");
        let s3 = s1 + &s2;
        //       ↑
        // value moved here.
        // println!("{:?}", s1);

        println!("{:?}", s2);
        println!("{:?}", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s3 = s1 + "-" + &s2 + "-" + &s3;
        // println!("s3: {:?}", s3);

        // 不會影響所有權
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }
    // String cp.2
    {
        let s = String::from("foo");
        // the type `String` cannot be indexed by `{integer}`
        // println!("{:?}", s[0]);
        println!("{:?}", s.len());
    }
    {
        let chinese = String::from("你好世界");
        println!("{}'s len: {}", chinese, chinese.len());

        for b in chinese.bytes() {
            println!("{}'s byte: {}", chinese, b)
        }
        for b in chinese.chars() {
            println!("{}'s chars: {}", chinese, b)
        }
    }
    // Hash Map
    {
        let mut scores: HashMap<String, usize> = HashMap::new();
        scores.insert(String::from("go"), 100);
    }
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let intial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
        println!("scores: HashMap = {:?}", scores);
    }
    {
        let key = String::from("key");
        let value = String::from("value");

        let mut map = HashMap::new();
        map.insert(key, value);
        map.insert(String::from("key2"), String::from("value2"));

        // ↓ key, value 的所有權已移動 , 不在生效
        // println!("{},{}", key, value);

        // 取值
        let m_key = map.get("key");
        if let Some(v) = m_key {
            println!("{:?}", v);
        } else {
            println!("None")
        }

        // 遍歷
        for (k, v) in &map {
            println!("{},{}", k, v);
        }
    }
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        // println!("{:?}", map);
        // ↓ format print line
        println!("{:#?}", map);
    }
}
