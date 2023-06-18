#[macro_use]
extern crate crossbeam;

use crossbeam::{channel::unbounded, internal::select};
use std::thread;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    let n_message = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            ConnectivityCheck::Ping => {
                responses_tx.send(ConnectivityCheck::Pong).unwrap();
            }
            ConnectivityCheck::Pong => {
                eprintln!("unexpected Pong response");
            }
            ConnectivityCheck::Pang => return,
        }
    });

    for _ in 0..n_message {
        requests_tx.send(ConnectivityCheck::Ping).unwrap();
    }

    requests_tx.send(ConnectivityCheck::Pang).unwrap();

    for _ in 0..n_message {
        select!(recv(responses_rx) -> msg => println!("{:?}", msg));
    }
}
