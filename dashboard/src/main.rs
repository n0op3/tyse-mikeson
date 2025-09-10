use std::{
    env,
    io::{Write, stdin, stdout},
    net::TcpStream,
};

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

    loop {
        let mut connection = connect();

        connection.write(
            encode(common::Packet::ImplantListRequest)
                .unwrap()
                .as_slice(),
        )?;

        let packet = read_packet(&mut connection).unwrap();
        let mut implants = Vec::new();

        match packet {
            Packet::ImplantList(implant_list) => {
                implants = implant_list;
            }
            _ => {}
        }

        println!("Available implants: ");
        for (i, implant) in implants.iter().enumerate() {
            println!(
                "[{}]: {} ({})",
                i + 1,
                implant.system_info.hostname,
                implant.system_info.name
            );
        }

        println!("");
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input == "exit" {
            break;
        }
    }

    Ok(())
}

fn connect() -> TcpStream {
    let address = env::var_os("TYSE_ADDRESS").unwrap();
    TcpStream::connect(address.into_string().unwrap()).unwrap()
}
