#[test]
fn guess() {
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("guess number!");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("read error");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("what you guess is {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too large"),
        }
    }
}
