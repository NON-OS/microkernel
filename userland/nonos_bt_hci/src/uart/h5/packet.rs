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

//! A whole Three-wire packet before SLIP framing: header, payload, and
//! the two-octet check when the header says one is present. A packet is
//! taken only when every part agrees: the header checksum holds, the
//! stated length is exactly the octets present, and the check, if any,
//! matches. Anything else is dropped (Part D, "Packet Header" and "Data
//! Integrity Check"); the reliable layer's retransmission recovers it.

use alloc::vec::Vec;

use super::crc::crc;
use super::header::{Header, PAYLOAD_MAX};

/// `h` and `payload` as one packet, with the check when `h.dic` is set;
/// `None` for a payload longer than the length field can state.
pub fn build(mut h: Header, payload: &[u8]) -> Option<Vec<u8>> {
    if payload.len() > PAYLOAD_MAX {
        return None;
    }
    h.len = payload.len() as u16;
    let mut p = Vec::with_capacity(4 + payload.len() + 2);
    p.extend_from_slice(&h.encode());
    p.extend_from_slice(payload);
    if h.dic {
        let c = crc(&p);
        p.extend_from_slice(&c.to_be_bytes());
    }
    Some(p)
}

/// Why a packet was not taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bad {
    Header,
    Length,
    Check,
}

/// The header and payload of `p`, if `p` is a whole, sound packet.
pub fn open(p: &[u8]) -> Result<(Header, &[u8]), Bad> {
    let head: [u8; 4] = p.get(..4).and_then(|h| h.try_into().ok()).ok_or(Bad::Header)?;
    let h = Header::parse(head).ok_or(Bad::Header)?;
    let end = 4 + usize::from(h.len);
    let want = end + if h.dic { 2 } else { 0 };
    if p.len() != want {
        return Err(Bad::Length);
    }
    if h.dic && crc(&p[..end]).to_be_bytes() != p[end..] {
        return Err(Bad::Check);
    }
    Ok((h, &p[4..end]))
}
