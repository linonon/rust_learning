// A hexdump clone with har-coded input that mocks file I/O

use std::io::{self, prelude::*};

const BYTES_PER_LINE: usize = 16;
// 多行字符串字头在使用原始字符串字头（r前缀和#分隔符）构建时不需要双引号转义。
// 额外的b前缀表明这应该被当作字节（&[u8]）而不是UTF-8文本（&str）。
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!");
}"#;

#[test]
fn test_7_3() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = io::Cursor::new(INPUT);
    cursor.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}]", position_in_input);
        for byte in line {
            print!("{:02x}", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
