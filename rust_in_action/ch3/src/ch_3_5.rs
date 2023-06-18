use std::fmt::Display;

#[test]
fn listing_3_9() {
    #[allow(dead_code)]
    enum Suit {
        Clubs,    // 梅花
        Spades,   // 黑桃
        Diamonds, // 钻石/方块
        Hearts,   // 红心
    }

    enum Card {
        King(Suit),
        Queen(Suit),
        Jack(Suit),
        Ace(Suit),
        Pipe(Suit, usize),
    }
}

#[test]
fn listing_3_10() {
    #[derive(Debug)]
    enum Event {
        Update,
        Delete,
        Unknown,
    }

    type Message = String;

    fn parse_log(line: &str) -> (Event, Message) {
        let parts: Vec<_> = line.splitn(2, ' ').collect();
        if parts.len() == 1 {
            return (Event::Unknown, String::from(line));
        }
        let event = parts[0];
        let rest = String::from(parts[1]);

        match event {
            "UPDATE" | "update" => return (Event::Update, String::from(rest)),
            "DELETE" | "delete" => return (Event::Delete, String::from(rest)),
            _ => return (Event::Unknown, String::from(line)),
        }
    }

    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}

#[test]
fn listing_3_14() {
    #[derive(Debug, PartialEq)]
    enum FileState {
        Open,
        Close,
    }

    impl Display for FileState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            match *self {
                FileState::Open => write!(f, "OPEN"),
                FileState::Close => write!(f, "CLOSE"),
            }
        }
    }

    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
        state: FileState,
    }

    impl File {
        fn new(name: &str) -> Self {
            Self {
                name: String::from(name),
                data: vec![],
                state: FileState::Close,
            }
        }
    }

    impl Display for File {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "<{} ({})> size:{}",
                self.name,
                self.state,
                self.data.len()
            )
        }
    }

    let a = FileState::Open;
    println!("{}", a);

    let f = File::new("linonon.txt");
    println!("{}", f);
    println!("{:?}", f)
}
