use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

#[test]
fn opening_a_file_in_rust_and_iterating_through_its_contents() {
    let arg1 = env::args().nth(1);

    let fname = arg1.unwrap_or_else(|| {
        "/home/linonon/Workspace/Rust-learning/rust_in_action/ch7-file-and-storage/Cargo.toml"
            .to_string()
    });
    println!("fname: {}", fname);

    let mut f = File::open(&fname).expect(&format!("cannot open {:?}", &fname));
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    println!("file name: {}", &fname);

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x}", byte),
            }
        }

        println!("");
        pos += BYTES_PER_LINE;
    }
}
