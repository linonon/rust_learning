use std::io::{BufRead, BufReader};

use clap::Command;

fn main() {
    listing_2_30_searching_through_a_file_or_stdin()
}

#[test]
fn listing_2_27_reading_a_file_manually_line_by_line() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let f = File::open("Cargo.toml").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        // Shinks the String back to length 0, preventing lines from persisting into the folloing ones
        line.truncate(0);
    }
}

#[test]
fn listing_2_28_reading_a_file_line_by_line_via_buf_reader_lines() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let f = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}

#[test]
fn test_2_30() {
    listing_2_30_searching_through_a_file_or_stdin()
}

fn listing_2_30_searching_through_a_file_or_stdin() {
    fn process_lines<T: BufRead>(reader: T, re: regex::Regex) {
        for line_ in reader.lines() {
            let line = line_.unwrap();
            match re.find(&line) {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }

    let args = Command::new("grep-lite")
        .version("0.1.0")
        .about("searches for patterns")
        .arg(
            clap::Arg::new("pattern")
                .help("The pattern to search fo")
                .required(true),
        )
        .arg(
            clap::Arg::new("input")
                .help("File to search")
                .required(false),
        )
        .get_matches();

    let pattern: &String = args.get_one("pattern").unwrap();
    let re = regex::Regex::new(pattern).unwrap();

    let none_input = String::from("-");
    let input: &String = args.get_one("input").unwrap_or(&none_input);

    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = std::fs::File::open(&input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
