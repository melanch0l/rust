use std::num::ParseIntError;

fn parse_port(input: &str) -> Result<u16, ParseIntError> {
    let port = input.parse::<u16>()?;
    Ok(port)
}

fn main() {
    for input in ["8080", "not-a-port"] {
        match parse_port(input) {
            Ok(port) => println!("valid port: {port}"),
            Err(error) => println!("could not parse {input:?}: {error}"),
        }
    }
}
