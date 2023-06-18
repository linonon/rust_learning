fn main() {
    println!("Hello, world!");
    // Stack: LIFO, 入站出站
    // Heap: 通過 指針訪問數據, 比 Stack 慢

    // 所有權, Rust 在編譯期間就會做好這一步, 則不需要運行時(rumtime)
    {
        // - 那跟蹤代碼的哪些部分正在使用 heap 的那些數據
        // - 最小化 heap 上的重複數據量
        // - 清理 heap 上未使用的數據以避免空間不足

        // 所有權規則
        // 每個值都有一個變量, 這個變量是該值的所有者
        // 每個值同時只能有一個所有者
        // 當所有者超出作用域(scpoe)時, 該值將被刪除
    }

    // String: heap 上分配
    {
        let mut s = String::from("hello");
        s.push_str(", world");
        println!("{}", s);
        // ↑ s 走出作用域後, 內存會自動釋放, drop()
    }

    // 變量和數據交互的方式: 移動(Move)
    // String 版本
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // ↑ 此時, s2 的 ptr 指向 s1 存在 heap 中的內容, 並廢棄 s1

        // ↓ 報錯: value borrowed here after move.
        // println!("s1: {}", s1);
        println!("s2: {}", s2);
    }

    // Clone: 針對 heap
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1: {}", s1);
        println!("s2: {}", s2);
    }

    // Copy: Stack
    {
        let x = 5;
        let y = x;
        println!("{}, {}", x, y);
    }

    // Copy Trait
    {
        // 簡單的標量都是可以 Copy 的
        // 需要分配內存的都是不能 Copy 的
        // 有 Copy trait 的類型:
        // - u32, i32... bool, char, f64, Tuple(內容是可 Copy 的)
    }

    // 所有權與函數
    {
        let s = String::from("hello");
        take_ownership(s);

        let x = 5;
        make_copy(x);
        println!("x: {}", x);

        fn take_ownership(s: String) {
            println!("take_ownership:{}", s);
        }
        fn make_copy(number: i32) {
            println!("make_copy:{}", number);
        }
    }

    // 返回值與作用域
    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);

        println!("s1: {}", s1);
        // Err: ↓ value borrowed here after move.
        // println!("s2: {}", s1);
        println!("s3: {}", s3);

        fn gives_ownership() -> String {
            String::from("hello")
        }
        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }

    /*
           4.2 引用
            - 引用的規則:
              - 一個可變的引用
              - 任意數量不可變的引用
            - 引用必須一直有效(避免懸空指針的問題)
    */
    {
        let mut s1 = String::from("hello");
        // &: 引用了 s1 的值, 但是不獲得 s1 的所有權
        let len = calculate_length(&mut s1);
        println!("len: {}", len);

        fn calculate_length(s: &mut String) -> usize {
            s.push_str("yes");
            s.len()
        }
    }
    // 可變引用:
    {
        let mut s = String::from("hello");
        let s1 = &mut s;
        // ↓ 會報錯, 防止數據競爭
        // Err: cannot borrow `s` as mutable more than once at a time.
        // let s2 = &mut s;
        // println!("{},{}", s1, s2);

        println!("{}", s1);
    }
    {
        // 不同作用域下 OK
        let mut s = String::from("hello");
        {
            let s1 = &mut s;
            println!("s1: {}", s1);
        }
        let s2 = &mut s;
        println!("s2: {}", s2);
    }

    /*
           4.3 切片
    */
    {
        let s = String::from("hello world");
        let index = first_word_index(&s);

        // s.clear() // -> index 與 s 不匹配
        println!("index: {}", index);
        fn first_word_index(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }

        let word = first_word(&s);
        println!("word: {}", word);
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                //
                if item == b' ' {
                    return &s[..i];
                }
            }
            &s[..]
            // s // ← 也可以?
        }
    }
    {
        let s = String::from("hello world");
        // let hello = &s[0..5];
        let hello = &s[..5];
        // let world = &s[6..11];
        let world = &s[6..];
        println!("{}, {}", hello, world);

        let whole = &s[..];
        println!("whole: {}", whole);
    }
}
