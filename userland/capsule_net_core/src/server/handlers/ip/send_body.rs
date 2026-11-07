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

//! What an OP_SEND_PACKET body must be before anything of it is used: the
//! IPv4 destination, the protocol, then a whole ICMP header of type, code,
//! checksum, identifier and sequence, and at most `MAX_PAYLOAD` bytes of
//! message. Any process holding net.ip's endpoint sends these bytes, so
//! each is checked here and never indexed past what arrived. Kept free of
//! smoltcp so net_core_proofs includes it unchanged.

use crate::protocol::ip::{E_BAD_LEN, E_BAD_PROTO, MAX_PAYLOAD, PROTO_ICMP};

/// The address and protocol ahead of the ICMP message.
const ADDR_PROTO: usize = 5;
/// An ICMP header: type, code, checksum, identifier, sequence.
pub const ICMP_HDR: usize = 8;

/// One echo a caller asked to send.
#[derive(Debug, PartialEq, Eq)]
pub struct Send<'a> {
    pub dst: [u8; 4],
    /// The identifier the ICMP socket is bound to.
    pub ident: u16,
    /// The whole ICMP message, header first.
    pub message: &'a [u8],
}

/// The send `body` asks for, or the error it is answered with.
pub fn read(body: &[u8]) -> Result<Send<'_>, u16> {
    if body.len() < ADDR_PROTO + ICMP_HDR || body.len() > ADDR_PROTO + MAX_PAYLOAD {
        return Err(E_BAD_LEN);
    }
    let (head, message) = body.split_at(ADDR_PROTO);
    if head[4] != PROTO_ICMP {
        return Err(E_BAD_PROTO);
    }
    let ident = u16::from_be_bytes([message[4], message[5]]);
    if ident == 0 {
        return Err(E_BAD_PROTO);
    }
    Ok(Send { dst: [head[0], head[1], head[2], head[3]], ident, message })
}
