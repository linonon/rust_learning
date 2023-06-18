//! # main
//! test
//! - ok?
//! - or not
//!

use ch14::mix;
use ch14::PrimaryColor;
fn main() {
    println!("Hello, world!");
    let x = 1;
    ch14::add_one(x);
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}
