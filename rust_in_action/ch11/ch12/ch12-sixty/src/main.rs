use std::process;
use std::thread::sleep;
use std::time;

fn main() {
    let delay = time::Duration::from_secs(1);

    let pid = process::id();
    println!("PID: {}", pid);

    // sleep 60 sec, print every second
    for i in 0..60 {
        println!("{} sec", i);
        sleep(delay);
    }
}
