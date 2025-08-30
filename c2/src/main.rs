use std::{io::Read, net::TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9120")?;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buf = vec![0; 1024];

        stream.read(&mut buf).expect("failed to read from stream");

        println!("{}", String::from_utf8_lossy(&buf));
    }

    Ok(())
}
