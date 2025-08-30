use bincode::{Decode, Encode};
use sysinfo::System;

#[derive(Debug, Encode, Decode)]
pub enum Packet {
    Beacon {
        timestamp: u128,
        system_info: SystemInfo,
    },
}

#[derive(Debug, Encode, Decode)]
pub struct SystemInfo {
    memory: u64,
    name: String,
    kernel: String,
    os_version: String,
    hostname: String,
}

impl SystemInfo {
    pub fn get() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            memory: (sys.total_memory() as f64 / 1024_u64.pow(3) as f64).ceil() as u64,
            name: System::name().unwrap_or("unknown system name".to_string()),
            kernel: System::kernel_long_version(),
            os_version: System::os_version().unwrap_or("unknown OS version".to_string()),
            hostname: System::host_name().unwrap_or("unknown hostname".to_string()),
        }
    }
}
