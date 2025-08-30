use std::{io::Write, net::TcpStream};

use bincode::config;
use common::{Packet, SystemInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = TcpStream::connect("127.0.0.1:9120")?;

    let packet = Packet::Beacon {
        system_info: SystemInfo::get(),
    };

    connection.write(
        bincode::encode_to_vec(packet, config::standard())
            .unwrap()
            .as_slice(),
    )?;

    Ok(())
}
