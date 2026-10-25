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

//! RNDIS messages, built and read (constants/rndis.rs has the numbers).
//! Pure: what a request says and what a reply or a received message means.

use alloc::vec::Vec;

use crate::constants::rndis::*;

fn word(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn put(v: &mut Vec<u8>, words: &[u32]) {
    for w in words {
        v.extend_from_slice(&w.to_le_bytes());
    }
}

/// INITIALIZE (rndis:319): request id, version, largest transfer.
pub fn initialize(request: u32) -> Vec<u8> {
    let mut v = Vec::new();
    put(&mut v, &[INITIALIZE_MSG, 24, request, MAJOR, MINOR, MAX_TRANSFER]);
    v
}

/// QUERY (rndis:371) of `oid` with no input: the information buffer's
/// offset counts from the request id, as SET's.
pub fn query(request: u32, oid: u32) -> Vec<u8> {
    let mut v = Vec::new();
    put(&mut v, &[QUERY_MSG, 28, request, oid, 0, 20, 0]);
    v
}

/// SET (rndis:396) of `oid` to `info`, which follows the request at offset
/// 20 from the request id.
pub fn set(request: u32, oid: u32, info: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    put(&mut v, &[SET_MSG, 28 + info.len() as u32, request, oid, info.len() as u32, 20, 0]);
    v.extend_from_slice(info);
    v
}

/// The station address as an RNDIS configuration parameter, as Linux's
/// rndis_filter_set_device_mac builds it: the name "NetworkAddress" and the
/// address as twelve lowercase hex digits, both UTF-16LE, after the
/// parameter's five-word header.
pub fn network_address(mac: &[u8; 6]) -> Vec<u8> {
    let name: Vec<u8> = NETWORK_ADDRESS.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let hex = b"0123456789abcdef";
    let value: Vec<u8> = mac
        .iter()
        .flat_map(|b| [hex[(b >> 4) as usize], hex[(b & 15) as usize]])
        .flat_map(|c| u16::from(c).to_le_bytes())
        .collect();
    let mut v = Vec::new();
    let name_at = 20u32;
    put(&mut v, &[name_at, name.len() as u32, PARAM_STRING, name_at + name.len() as u32, value.len() as u32]);
    v.extend_from_slice(&name);
    v.extend_from_slice(&value);
    v
}

/// The data message carrying `frame`, its header first (rndis:507): the
/// data right after the header, no out-of-band data, no per-packet info.
pub fn packet_header(frame_len: usize) -> [u8; PACKET_HEADER] {
    let mut h = [0u8; PACKET_HEADER];
    let words = [PACKET_MSG, (PACKET_HEADER + frame_len) as u32, 36, frame_len as u32];
    for (i, w) in words.iter().enumerate() {
        h[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
    }
    h
}

/// What a message from the host is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Received {
    /// A reply to the request with this id, its status, and where its
    /// information buffer lies in the message (QUERY's; empty otherwise).
    Reply { request: u32, status: u32, info: (usize, usize) },
    /// A frame, at this offset and length in the message.
    Frame(usize, usize),
    /// The link went up (true) or down.
    Link(bool),
    /// Anything else: another status, a message type this driver never
    /// asks for.
    Other,
}

/// Read one message the host put in the receive buffer, held to its own
/// length and to the `len` bytes it was given in.
pub fn read(m: &[u8]) -> Option<Received> {
    let ty = word(m, 0)?;
    let len = (word(m, 4)? as usize).min(m.len());
    let m = &m[..len];
    Some(match ty {
        INITIALIZE_CMPLT | SET_CMPLT => Received::Reply { request: word(m, 8)?, status: word(m, 12)?, info: (0, 0) },
        QUERY_CMPLT => {
            let (n, off) = (word(m, 16)? as usize, word(m, 20)? as usize);
            let at = 8usize.checked_add(off)?;
            if at.checked_add(n)? > m.len() {
                return None;
            }
            Received::Reply { request: word(m, 8)?, status: word(m, 12)?, info: (at, n) }
        }
        PACKET_MSG => {
            let (off, n) = (word(m, 8)? as usize, word(m, 12)? as usize);
            let at = 8usize.checked_add(off)?;
            if n == 0 || at.checked_add(n)? > m.len() {
                return None;
            }
            Received::Frame(at, n)
        }
        INDICATE_STATUS_MSG => match word(m, 8)? {
            STATUS_MEDIA_CONNECT => Received::Link(true),
            STATUS_MEDIA_DISCONNECT => Received::Link(false),
            _ => Received::Other,
        },
        _ => Received::Other,
    })
}
