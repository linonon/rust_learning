use std::env;

use ch2_2to2_3::gcd::gcd;

/// ## 處理命令行參數, 並輸出最大公因數
fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers
            .push(u64::from_str_radix(&arg, 10).expect(&format!("error parsing argument: {}", arg)))
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m)
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d)
}
