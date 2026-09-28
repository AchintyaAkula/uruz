use crate::lynx::commands::Command;
use crate::lynx::error::PacketError;
use std::sync::atomic::{AtomicU8, Ordering};

static PKT_ID_ASSIGNEE: AtomicU8 = AtomicU8::new(0);

/// Packet Struct
/// Represents standard lynx datagram sent by the Android/Java layer
///
/// Checksum is not a field because it is automatically applied when exporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    header: [u8; 2],  // D, K
    pkt_length: u16,  // 11+ bytes
    target_addr: u8,  // 0x01 or 0x02 or 0x03
    src_addr: u8,     // 0x00 usually
    pkt_id: u8,       // Wrapping tracking num from 0->255
    ref_id: u8,       // Reference num for replies, all outbound have 0 by default
    pkt_type: u16,    // Type of message/command
    payload: Vec<u8>, // Actual message data
//  checksum: u8,     // CRC Checksum
}

impl Packet {
    pub fn new(target_addr: u8, pkt_type: impl Command, payload: Vec<u8>) -> Self {
        Packet {
            pkt_length: (11 + payload.len()) as u16,
            target_addr,
            pkt_type: pkt_type.into(),
            payload,
            ..Packet::default()
        }
    }

    /// Imports into `Packet` struct using a `Vec<u8>`
    ///
    /// Verifies Checksum and Header and handles errors
    pub fn import(data: Vec<u8>) -> Result<Self, PacketError> {
        if !(data[0] == 0x44 && data[1] == 0x4B) {
            Err(PacketError::InvalidHeader)
        } else if !Self::verify_checksum(data.clone()) {
            Err(PacketError::InvalidChecksum)
        } else {
            Ok(Packet {
                pkt_length: u16::from_le_bytes(data[2..=3].try_into().unwrap()),
                target_addr: data[4],
                src_addr: data[5],
                pkt_id: data[6],
                ref_id: data[7],
                pkt_type: u16::from_le_bytes(data[8..=9].try_into().unwrap()),
                payload: data[10..data.len() - 1].into(),
                ..Self::default()
            })
        }
    }

    /// Exports `Packet` struct into `Vec<u8`
    ///
    /// Calculates checksum, and assigns unique packet ID
    pub fn export(&mut self) -> Vec<u8> {
        self.pkt_id = PKT_ID_ASSIGNEE.fetch_add(1, Ordering::Relaxed);

        let mut bytes: Vec<u8> = Vec::with_capacity(self.pkt_length as usize);
        let mut sum: u8 = 0u8;

        for &b in &self.header {
            sum = sum.wrapping_add(b);
            bytes.push(b);
        }
        for &b in &self.pkt_length.to_le_bytes() {
            sum = sum.wrapping_add(b);
            bytes.push(b);
        }
        sum = sum
            .wrapping_add(self.target_addr)
            .wrapping_add(self.src_addr)
            .wrapping_add(self.pkt_id)
            .wrapping_add(self.ref_id);
        bytes.push(self.target_addr);
        bytes.push(self.src_addr);
        bytes.push(self.pkt_id);
        bytes.push(self.ref_id);
        for &b in &self.pkt_type.to_le_bytes() {
            sum = sum.wrapping_add(b);
            bytes.push(b);
        }
        for &b in &self.payload {
            sum = sum.wrapping_add(b);
            bytes.push(b);
        }
        bytes.push(sum);
        bytes
    }

    fn verify_checksum(data: Vec<u8>) -> bool {
        let mut checksum: u8 = 0u8;
        for b in 0..data.len() {
            checksum = checksum.wrapping_add(data[b]);
        }
        checksum == *data.last().unwrap()
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self {
            header: [0x44, 0x4b],
            pkt_length: 0x0b,
            target_addr: 0x01,
            src_addr: 0x00,
            pkt_id: 0x00,
            ref_id: 0x00,
            pkt_type: 0x00,
            payload: Vec::new(),
        }
    }
}
