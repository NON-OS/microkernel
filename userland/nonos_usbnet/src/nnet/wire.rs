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

//! "NNET", the NIC protocol net.core and net.l2 speak with every wired and
//! Wi-Fi driver (capsule_driver_rtl8139/src/protocol): a 20-byte header,
//! then in a reply a status word and the op's payload.

pub const MAGIC: u32 = 0x4E4E_4554;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
/// Where a reply's payload starts.
pub const DATA_AT: usize = HDR_LEN + STATUS_LEN;
/// A received frame's payload: its length, then its bytes.
pub const RX_PREFIX_LEN: usize = 4;
pub const STATS_LEN: usize = 48;

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_LINK_STATUS: u16 = 2;
pub const OP_MAC_ADDRESS: u16 = 3;
pub const OP_TX_PACKET: u16 = 4;
pub const OP_RX_PACKET: u16 = 5;
pub const OP_STATS: u16 = 6;

pub const E_INVAL: i32 = -22;
pub const E_IO: i32 = -5;
pub const E_AGAIN: i32 = -11;
pub const E_MSGSIZE: i32 = -90;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Request {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

/// The request at the start of `buf`, when it is one of ours.
pub fn decode(buf: &[u8]) -> Option<Request> {
    if buf.len() < HDR_LEN {
        return None;
    }
    let word = |at: usize| u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]]);
    let half = |at: usize| u16::from_le_bytes([buf[at], buf[at + 1]]);
    if word(0) != MAGIC || half(4) != VERSION {
        return None;
    }
    Some(Request { op: half(6), flags: half(8), request_id: word(12), payload_len: word(16) })
}
