use std::{fs::File, io, net::Ipv6Addr};

// fn main() -> Result<(), Box<dyn std::error::Error>>{
//     let _f = File::open("hello.txt")?;

//     let _localhost = "::1".parse::<Ipv6Addr>()?;

// 	Ok(())
// }

#[derive(Debug)]
enum UpstreamError {
    Io(std::io::Error),
    Parse(std::net::AddrParseError),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for UpstreamError {
    fn from(err: std::io::Error) -> Self {
        UpstreamError::Io(err)
    }
}

impl From<std::net::AddrParseError> for UpstreamError {
    fn from(err: std::net::AddrParseError) -> Self {
        UpstreamError::Parse(err)
    }
}

fn main() -> Result<(), UpstreamError> {
    // Using map_err
    let _f = File::open("hello.txt").map_err(UpstreamError::Io)?;
    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parse)?;

    // After impl `From` trait
    let _f = File::open("hello.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
