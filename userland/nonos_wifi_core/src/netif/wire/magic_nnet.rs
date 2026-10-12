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

//! The net_core link protocol as the driver side sees it. net_core drives every
//! network interface, wired or wireless, with the same 20-byte envelope and
//! four operations; a WiFi driver becomes a bound interface by answering them.
//! These constants and helpers mirror capsule_net_core/src/protocol, kept local
//! so the adapter stays self-contained.

/// "NNET" little-endian, the tag on every net_core link message.
pub const MAGIC_NNET: u32 = 0x4E4E_4554;

pub const VERSION: u16 = 1;

pub const HDR_LEN: usize = 20;

pub const OP_LINK_STATUS: u16 = 2;

pub const OP_MAC_ADDRESS: u16 = 3;

pub const OP_TX_PACKET: u16 = 4;

pub const OP_RX_PACKET: u16 = 5;

/// The largest 802.3 frame carried in either direction (1500 MTU + header).
pub const MAX_FRAME: usize = 1514;

/// Status meaning "no frame right now"; net_core treats any nonzero receive
/// status as an empty poll and retries.
pub const STATUS_AGAIN: i32 = 8;

/// Status meaning "the link is down" (not associated).
pub const STATUS_NO_LINK: i32 = 5;

/// A parsed request: its operation, the request id to echo, and the payload
/// bytes after the header (a frame for a transmit, empty otherwise).
pub struct Request<'a> {
    pub op: u16,
    pub request_id: u32,
    pub payload: &'a [u8],
}
