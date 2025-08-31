use std::{io::Write, net::TcpStream};

use common::{Packet, SystemInfo, encode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = TcpStream::connect("127.0.0.1:9120")?;

    let packet = Packet::Beacon {
        system_info: SystemInfo::get(),
    };

    connection.write(encode(&packet).unwrap().as_slice())?;

    Ok(())
}
