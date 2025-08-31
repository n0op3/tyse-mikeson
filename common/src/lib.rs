use bincode::{Decode, Encode};
use sysinfo::System;

pub const IMPLANT_TIMEOUT_SECONDS: u64 = 60 * 60;

#[derive(Debug, Encode, Decode)]
pub enum Packet {
    Beacon { system_info: SystemInfo },
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct SystemInfo {
    memory_gb: u64,
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
            memory_gb: (sys.total_memory() as f64 / 1024_u64.pow(3) as f64).ceil() as u64,
            name: System::name().unwrap_or("unknown system name".to_string()),
            kernel: System::kernel_long_version(),
            os_version: System::os_version().unwrap_or("unknown OS version".to_string()),
            hostname: System::host_name().unwrap_or("unknown hostname".to_string()),
        }
    }
}
