use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum Packet {
    Ping { timestamp: u128 },
}
