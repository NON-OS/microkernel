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

//! What a transfer puts in the host's registers and takes back out, decided
//! from the request alone. The registers are the ICH and PCH datasheets'
//! SMBus I/O block: HST_ADD, HST_CMD, HST_D0, HST_D1 and HST_CNT. Pure, so
//! the proofs can hold every protocol's shape.

use super::kind::Kind;
use super::request::Xfer;
use crate::regs::CNT_START;

/// The data a protocol moves through the host besides its command byte.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Data {
    None,
    /// One byte in HST_D0.
    Low,
    /// Two bytes, low in HST_D0 and high in HST_D1.
    Pair,
    /// Up to 32 bytes through the block buffer, the count in HST_D0.
    Buffer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Plan {
    /// HST_ADD: the 7-bit address in bits 7:1, bit 0 set for a read.
    pub address: u8,
    /// HST_CMD, for every protocol that sends a byte after the address.
    pub command: Option<u8>,
    /// Loaded before START for a write, read out after the end for a read.
    pub data: Data,
    pub read: bool,
    /// HST_CNT: the protocol in SMB_CMD (bits 4:2) with START (bit 6).
    pub start: u8,
}

pub fn plan(x: &Xfer) -> Plan {
    // A send-byte's one byte rides in HST_CMD; a receive-byte has no
    // command and its byte comes back in HST_D0.
    let (command, data) = match (x.kind, x.read) {
        (Kind::Quick, _) => (None, Data::None),
        (Kind::Byte, false) => (Some(x.cmd), Data::None),
        (Kind::Byte, true) => (None, Data::Low),
        (Kind::ByteData, _) => (Some(x.cmd), Data::Low),
        (Kind::WordData, _) => (Some(x.cmd), Data::Pair),
        (Kind::Block, _) => (Some(x.cmd), Data::Buffer),
    };
    Plan {
        address: x.addr << 1 | x.read as u8,
        command,
        data,
        read: x.read,
        start: x.kind.control() | CNT_START,
    }
}
