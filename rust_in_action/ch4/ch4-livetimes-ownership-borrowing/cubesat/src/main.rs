#![allow(unused, dead_code)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status_v1(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn check_status_v2(sat_id: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn check_status_v3(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    // v1 (: u64)
    let sat_a = 0;

    let a_status = check_status_v1(sat_a);
    println!("a: {:?}", a_status);

    // "waiting" ...
    let a_status = check_status_v1(sat_a);
    println!("a: {:?}", a_status);

    //
    // v2 (: &CubeSat)
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status_v2(&sat_a);
    println!("a: {:?}", a_status);

    // "waiting" ...
    let a_status = check_status_v2(&sat_a);
    println!("a: {:?}", a_status);

    //
    // v3 -> CubeSat
    let sat_a = CubeSat { id: 0 };

    let sat_a = check_status_v3(sat_a);

    // "waiting" ...
    let sat_a = check_status_v3(sat_a);
}
