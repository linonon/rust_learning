use std::{
    fs::File,
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

fn main() {
    println!("Hello, world!");

    {
        // panic!("crach and burn")

        let v = vec![1, 2, 3];
        println!("{:?}", v);

        // v[99];
        // ↑ panic
    }
    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Error creating file: {:?}", e),
                },
                other => panic!("Error opening file: {:?}", other),
            },
        };
        println!("{:?}", f);
    }
    // {
    //     //unwarp
    //     // let f = File::open("unwarp.txt").unwrap();
    //     let f = File::open("unwarp.txt").expect("Error opening file: unwarp.txt");
    // }
    {
        let result = read_username_from_file();
        println!("{:?}", result);

        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello3.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
    }
    {
        let result = read_username_from_file();
        println!("{:?}", result);

        fn read_username_from_file() -> Result<String, io::Error> {
            // let mut f = File::open("hello4.txt")?;
            // let mut s = String::new();
            // f.read_to_string(&mut s)?;
            // Ok(s)
            let mut s = String::new();
            // "?" 只適用於返回類型為 Result 的函數
            File::open("hello4.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    // 什麼時候用 panic!
    {
        // 如果可以確定 Result 就是 Ok 時
        let home: IpAddr = "127.0.0.1".parse().unwrap();
        println!("home = {}", home)
    }
}
