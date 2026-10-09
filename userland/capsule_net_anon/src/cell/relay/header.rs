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

//! The relay message header, laid into a payload and read back out.

use super::super::geometry::{
    PAYLOAD_BYTES, RELAY_BODY_BYTES, RELAY_HEADER_BYTES, RELAY_LENGTH_AT,
};

/// The header on a relay message, inside the onion layers.
pub struct RelayHeader {
    pub command: u8,
    /// Zero when the cell is for us. A relay that is only forwarding sets it to
    /// something else, and this is how a hop knows a cell is not its own.
    pub recognized: u16,
    pub stream: u16,
    pub length: u16,
}

/// Lay a relay message into a cell payload, integrity left zero for the digest
/// step to fill in. `None` if the body is longer than a cell can carry.
pub fn pack(header: &RelayHeader, body: &[u8]) -> Option<[u8; PAYLOAD_BYTES]> {
    if body.len() > RELAY_BODY_BYTES {
        return None;
    }
    let mut out = [0u8; PAYLOAD_BYTES];
    out[0] = header.command;
    out[1..3].copy_from_slice(&header.recognized.to_be_bytes());
    out[3..5].copy_from_slice(&header.stream.to_be_bytes());
    out[RELAY_LENGTH_AT..RELAY_LENGTH_AT + 2].copy_from_slice(&(body.len() as u16).to_be_bytes());
    out[RELAY_HEADER_BYTES..RELAY_HEADER_BYTES + body.len()].copy_from_slice(body);
    Some(out)
}

/// Read the header fields. Total and infallible: every field is at a fixed
/// offset inside a payload that is always the same length.
pub fn unpack(payload: &[u8; PAYLOAD_BYTES]) -> RelayHeader {
    RelayHeader {
        command: payload[0],
        recognized: u16::from_be_bytes([payload[1], payload[2]]),
        stream: u16::from_be_bytes([payload[3], payload[4]]),
        length: u16::from_be_bytes([payload[RELAY_LENGTH_AT], payload[RELAY_LENGTH_AT + 1]]),
    }
}

/// The message body, or `None` when the stated length runs past the cell.
///
pub fn body(payload: &[u8; PAYLOAD_BYTES]) -> Option<&[u8]> {
    let length = unpack(payload).length as usize;
    if length > RELAY_BODY_BYTES {
        return None;
    }
    Some(&payload[RELAY_HEADER_BYTES..RELAY_HEADER_BYTES + length])
}
