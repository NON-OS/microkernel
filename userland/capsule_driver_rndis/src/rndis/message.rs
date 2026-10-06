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

//! The RNDIS messages, statuses and object identifiers the host side
//! sends and reads (Remote NDIS 1.0, section 2; Linux include/linux/rndis.h
//! and QEMU hw/usb/dev-network.c give the same numbers), and the
//! little-endian words every message is made of.

/// The class requests that carry them on endpoint 0 (CDC 1.2, 6.2.1, 6.2.2).
pub const SEND_ENCAPSULATED_COMMAND: u8 = 0x00;
pub const GET_ENCAPSULATED_RESPONSE: u8 = 0x01;

pub const MSG_PACKET: u32 = 1;
pub const MSG_INIT: u32 = 2;
pub const MSG_HALT: u32 = 3;
pub const MSG_QUERY: u32 = 4;
pub const MSG_SET: u32 = 5;
pub const MSG_INDICATE: u32 = 7;
/// A completion carries its request's type with the top bit set.
pub const COMPLETION: u32 = 0x8000_0000;

pub const STATUS_SUCCESS: u32 = 0;
pub const MEDIUM_802_3: u32 = 0;

pub const OID_802_3_PERMANENT_ADDRESS: u32 = 0x0101_0101;
pub const OID_GEN_CURRENT_PACKET_FILTER: u32 = 0x0001_010E;
/// NDIS_PACKET_TYPE_DIRECTED, ALL_MULTICAST and BROADCAST. Linux
/// RNDIS_DEFAULT_FILTER adds PROMISCUOUS; the stack needs only these.
pub const FILTER: u32 = 0x01 | 0x04 | 0x08;

/// The errnos a control exchange ends with: no answer in time, and an
/// answer whose status is not success (Linux rndis_command's -ETIMEDOUT
/// and -EL3RST).
pub const E_TIMEDOUT: i32 = -110;
pub const E_REFUSED: i32 = -47;

/// The little-endian word at `at`, when `b` holds all four bytes.
pub fn le32(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

/// Write `v` at `at`; the callers' buffers are sized for their layouts.
pub fn put32(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes());
}
