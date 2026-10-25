// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The Three-wire UART packet header (Vol 4, Part D, "Packet Header"):
//! four octets.
//! - Octet 0: the sequence number in bits 0-2, the acknowledgement number
//!   in bits 3-5, data integrity check present in bit 6, and reliable
//!   packet in bit 7.
//! - Octet 1: the packet type in bits 0-3, and the payload length's low
//!   four bits in bits 4-7.
//! - Octet 2: the payload length's high eight bits.
//! - Octet 3: the header checksum, chosen so all four octets sum to 0xFF
//!   modulo 256.

/// Acknowledgement packet: no payload, carries only the ack number.
pub const ACK: u8 = 0;
pub const COMMAND: u8 = 1;
pub const ACL: u8 = 2;
pub const SYNC_DATA: u8 = 3;
pub const EVENT: u8 = 4;
/// Link establishment and low power messages.
pub const LINK_CONTROL: u8 = 15;
/// The longest payload the 12-bit length can state.
pub const PAYLOAD_MAX: usize = 0x0fff;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Header {
    pub seq: u8,
    pub ack: u8,
    pub dic: bool,
    pub reliable: bool,
    pub kind: u8,
    pub len: u16,
}

impl Header {
    /// The four octets; the numbers are taken modulo 8 and the type and
    /// length to their field widths.
    pub fn encode(&self) -> [u8; 4] {
        let b0 = self.seq & 7
            | (self.ack & 7) << 3
            | u8::from(self.dic) << 6
            | u8::from(self.reliable) << 7;
        let b1 = self.kind & 0x0f | ((self.len & 0x0f) as u8) << 4;
        let b2 = ((self.len >> 4) & 0xff) as u8;
        [b0, b1, b2, !b0.wrapping_add(b1).wrapping_add(b2)]
    }

    /// The header in `h`, or `None` when its checksum does not hold.
    pub fn parse(h: [u8; 4]) -> Option<Self> {
        let sum = h.iter().fold(0u8, |s, &b| s.wrapping_add(b));
        (sum == 0xff).then_some(Self {
            seq: h[0] & 7,
            ack: h[0] >> 3 & 7,
            dic: h[0] & 0x40 != 0,
            reliable: h[0] & 0x80 != 0,
            kind: h[1] & 0x0f,
            len: u16::from(h[1] >> 4) | u16::from(h[2]) << 4,
        })
    }
}
