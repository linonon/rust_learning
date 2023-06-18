use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secert number is: {}", secret_number);

    loop {
        println!("guess a number");
        let mut guess = String::new();

        // io::Result OK, Err
        io::stdin().read_line(&mut guess).expect("not found");

        // shadow
        // trim() delete " ", "\n" ...
        // parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("what you guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), // arm 分支
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("OK");
                break;
            }
        }
    }
}
