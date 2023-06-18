use std::io::Read;

#[allow(unused)]
use crate::pkg::*;

#[test]
fn result_enum_part_one() {
    use std::fs::File;

    __("open file 演示错误处理");
    let file_path: &str =
        "/home/linonon/Workspace/Rust-learning/rust_basic_bilibili_review/src/ch9_error/file.txt";
    let _ = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file {:?}", err),
    };

    __("open file 演示错误处理 Version 2");
    comment("很多 match , 很原始");
    let _ = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => handle_open_error(err, file_path),
    };

    fn handle_open_error(err: std::io::Error, path: &str) -> std::fs::File {
        match err.kind() {
            std::io::ErrorKind::NotFound => create_file(path),
            other => panic!("error opening file: {}", other),
        }
    }
    fn create_file(path: &str) -> std::fs::File {
        match std::fs::File::create(path) {
            Ok(fc) => fc,
            Err(err) => panic!("error creating file: {}", err),
        }
    }

    __("open file 演示错误处理 Version 3");
    let f = File::open(file_path).unwrap_or_else(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|err| {
                panic!("error creating file: {}", err);
            })
        } else {
            panic!("error opening file: {}", err);
        }
    });

    __("unwrap");

    __("");
}

#[test]
fn result_enum_part_two() {
    use std::fs::File;

    __("Result<_,_> 传播错误");
    let path: &str =
        "/home/linonon/Workspace/Rust-learning/rust_basic_bilibili_review/src/ch9_error/file.txt";
    fn read_username_from_file(path: &str) -> Result<String, std::io::Error> {
        let f = File::open(path);
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

    let f = read_username_from_file(path);
    match f {
        Ok(s) => println!("read username from file = {}", s),
        Err(e) => panic!("error read from file = {}", e),
    }

    __("使用 '?' 传播错误");
    fn read_username_from_file_v2(path: &str) -> Result<String, std::io::Error> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        Ok(s)
    }

    let f = read_username_from_file_v2(path);
    match f {
        Ok(s) => println!("read username from file = {}", s),
        Err(e) => panic!("error read from file = {}", e),
    }

    __("'?' 与 from 函数");
    comment("Trait std::convert::From 上的 from 函数:");
    comment("被 '?' 所应用的错误 , 会隐式的被 from 函数处理");
    comment("当 '?' 调用 from 函数时:");
    comment_lv2("如果 Output = typeA 的话, 则会将 typeB 转化成 typeA");

    __("链式调用");
    fn read_username_from_file_v3(path: &str) -> Result<String, std::io::Error> {
        let mut s = String::new();
        File::open(path)?.read_to_string(&mut s)?;

        Ok(s)
    }

    __("");
}
