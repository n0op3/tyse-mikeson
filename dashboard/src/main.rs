use std::{env, io::Write, net::TcpStream};

use common::{Packet, encode, read_packet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passwd = rpassword::prompt_password("Input password: ")?;

    let mut connection = connect();
    connection.write(
        encode(Packet::DashboardLogin { password: passwd })
            .unwrap()
            .as_slice(),
    )?;

    let packet = read_packet(&mut connection).unwrap();

    match packet {
        Packet::LoginSuccess => {
            println!("Login successful!");
        }
        Packet::LoginFailed => {
            println!("Login failed.");
            return Ok(());
        }
        _ => {
            println!("Unexpected packet received");
            return Ok(());
        }
    }

    let mut connection = connect();

    connection.write(
        encode(common::Packet::ImplantListRequest)
            .unwrap()
            .as_slice(),
    )?;

    let packet = read_packet(&mut connection).unwrap();

    match packet {
        Packet::ImplantList(implants) => {
            println!("{:?}", implants);
        }
        _ => {}
    }

    Ok(())
}

fn connect() -> TcpStream {
    let address = env::var_os("TYSE_ADDRESS").unwrap();
    TcpStream::connect(address.into_string().unwrap()).unwrap()
}
