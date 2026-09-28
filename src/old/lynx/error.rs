use std::error::Error;
use std::fmt::{Display, Formatter, Result as Res};

#[derive(Debug)]
pub enum PacketError {
    InvalidHeader,
    InvalidChecksum,
}

impl Display for PacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Res {
        match self {
            PacketError::InvalidHeader => write!(f, "Invalid header, expected \"0x44, 0x4B\""),
            PacketError::InvalidChecksum => write!(f, "Invalid checksum, packet is invalid"),
        }
    }
}

impl Error for PacketError {}
