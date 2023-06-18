use std::{env, process};

use ch12::Config;
fn main() {
    // cargo run substring xxx.txt

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ch12::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
