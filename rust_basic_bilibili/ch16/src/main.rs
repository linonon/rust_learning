use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned i: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main i: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    match handle.join() {
        Ok(_) => println!("finished"),
        Err(_) => println!("interrupted"),
    };
}

#[test]
fn move_closuer() {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("vector: {:?}", v);
    })
    .join()
    .unwrap();
}

#[test]
fn channel_test() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[test]
fn channel_test_2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5, 10, 9, 8, 7, 6];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    println!("send done");
    loop {
        let val = rx.try_recv();
        match val {
            Ok(v) => {
                if v == 10 {
                    break;
                }
                println!("Got: {}", v);
            }
            _ => (),
        }
    }
}

#[test]
/// ## RefCell / Rc VS Mutex / Arc
/// - Mutex 提供了內部可變性, 和 Cell 家族一樣
/// - 我們使用 RefCell 來改變 Rc 裡的內容 => 循環引用的風險
/// - 我們使用 Mutex 來改變 Arc 裡面的內容 => 死鎖的風險
fn mutex_test() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())
}
