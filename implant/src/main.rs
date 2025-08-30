use std::{
    io::Write,
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
};

use bincode::config;
use common::Packet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = TcpStream::connect("127.0.0.1:9120")?;

    let packet = Packet::Ping {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    };

    connection.write(
        bincode::encode_to_vec(packet, config::standard())
            .unwrap()
            .as_slice(),
    )?;

    Ok(())
}
