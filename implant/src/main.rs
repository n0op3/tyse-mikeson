use std::{io::Write, net::TcpStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = TcpStream::connect("127.0.0.1:9120")?;

    connection.write("Hello, world!".as_bytes())?;

    Ok(())
}
