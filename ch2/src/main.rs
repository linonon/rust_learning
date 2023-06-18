use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secert number is: {}", secret_number);
    println!("guess a number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("not found");
    // io::Result OK, Err

    println!("what you guess: {}", guess);


    match guess.cmp(&secret_number) {
        
    }
}
