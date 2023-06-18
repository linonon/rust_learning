use std::fs::OpenOptions;

#[test]
fn open_options() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("test.txt")
        .unwrap();
}
