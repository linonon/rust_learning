#[allow(unused)]
static mut ERROR: i32 = 0;

#[test]
fn test_main() {
    use rand::random;
    use std::io::Read;

    #[derive(Debug)]
    pub struct File {
        name: String,
        data: Vec<u8>,
    }

    impl File {
        pub fn new(name: &str) -> Self {
            Self {
                name: String::from(name),
                data: vec![],
            }
        }

        #[allow(dead_code)]
        pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
            let mut f = File::new(name);
            f.data = data.clone();
            f
        }

        fn read(&mut self, save_to: &mut Vec<u8>) -> usize {
            let mut tmp = self.data.clone();
            let read_length = tmp.len() as usize;
            // HACK: 自定义新增长度, 避免过多调用增长函数
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            read_length
        }
    }

    fn open(f: &mut File) -> bool {
        match std::fs::File::open(f.name.as_str()) {
            Ok(mut res) => {
                res.read_to_end(&mut f.data).unwrap();
                true
            }
            Err(e) => {
                println!("read file error: {e}");
                false
            }
        }
    }

    fn close(_f: &mut File) -> bool {
        if random() && random() && random() {
            unsafe {
                ERROR = 1;
            }
        }
        true
    }

    // main()
    // let mut f2 = File {
    //     name: String::from(
    //         "/home/linonon/Workspace/Rust-learning/rust_in_action/cp3/file/2.txt",
    //     ),
    //     data: vec![],
    // };
    let mut f2 = File::new("/home/linonon/Workspace/Rust-learning/rust_in_action/cp3/file/2.txt");

    let mut buf: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = f2.read(&mut buf);
    close(&mut f2);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured")
        }
    }

    let text = String::from_utf8_lossy(&buf);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text)
}
