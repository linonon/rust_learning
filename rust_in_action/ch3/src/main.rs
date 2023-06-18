//! Simulating files one step at a time

mod ch_3_1;
mod ch_3_5;

fn main() {}

/// Represents a "file",
/// whick probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    // New files are assumed to be empty, but a name is required.
    pub fn new(name: String, data: Vec<u8>) -> File {
        File { name, data }
    }
}
