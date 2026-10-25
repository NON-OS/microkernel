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

//! The network VSP's messages, built and read (constants/nvsp.rs has the
//! numbers). Pure.

use crate::constants::nvsp::*;

pub type Message = [u8; MESSAGE];

fn word(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// A message of type `ty` whose fields are `words`, zero padded.
pub fn message(ty: u32, words: &[u32]) -> Message {
    let mut m = [0u8; MESSAGE];
    m[..4].copy_from_slice(&ty.to_le_bytes());
    for (i, w) in words.iter().enumerate() {
        m[4 + 4 * i..8 + 4 * i].copy_from_slice(&w.to_le_bytes());
    }
    m
}

/// INIT for one version, as both its fields (Linux's negotiate_nvsp_ver).
pub fn init(version: u32) -> Message {
    message(INIT, &[version, version])
}

/// SEND_NDIS_CONFIG: the MTU, a reserved word, no capability asked.
pub fn ndis_config() -> Message {
    message(SEND_NDIS_CONFIG, &[MTU, 0, 0, 0])
}

pub fn ndis_version((major, minor): (u32, u32)) -> Message {
    message(SEND_NDIS_VERSION, &[major, minor])
}

/// SEND_RECEIVE_BUFFER or SEND_SEND_BUFFER: the GPADL and the buffer's id.
pub fn buffer(ty: u32, gpadl: u32, id: u16) -> Message {
    message(ty, &[gpadl, u32::from(id)])
}

/// SEND_RNDIS_PACKET: which RNDIS channel, and the send buffer section the
/// message is in, with its length.
pub fn rndis_packet(channel: u32, section: u32, len: u32) -> Message {
    message(SEND_RNDIS_PACKET, &[channel, section, len])
}

/// SEND_RNDIS_PACKET_COMPLETE, success: the guest's answer to a receive.
pub fn rndis_done() -> Message {
    message(SEND_RNDIS_PACKET_COMPLETE, &[STATUS_SUCCESS])
}

/// Whether INIT_COMPLETE took the version (proto:251-254: its status is
/// the third field).
pub fn init_taken(b: &[u8]) -> bool {
    word(b, 0) == Some(INIT_COMPLETE) && word(b, 12) == Some(STATUS_SUCCESS)
}

/// The receive buffer's one section (proto:290-307): its sub-allocation
/// size and how many there are, when the host took the buffer.
pub fn receive_buffer(b: &[u8]) -> Option<(u32, u32)> {
    if word(b, 0)? != SEND_RECEIVE_BUFFER_COMPLETE || word(b, 4)? != STATUS_SUCCESS || word(b, 8)? == 0 {
        return None;
    }
    // The section: offset, sub-allocation size, how many, end offset.
    let (offset, size, count) = (word(b, 12)?, word(b, 16)?, word(b, 20)?);
    (offset == 0 && size > 0 && count > 0).then_some((size, count))
}

/// The send buffer's section size (proto:341-350), when the host took it.
pub fn send_buffer(b: &[u8]) -> Option<u32> {
    if word(b, 0)? != SEND_SEND_BUFFER_COMPLETE || word(b, 4)? != STATUS_SUCCESS {
        return None;
    }
    word(b, 8).filter(|s| *s > 0)
}

/// A SEND_RNDIS_PACKET the host sent with a receive: its RNDIS channel.
pub fn received(b: &[u8]) -> Option<u32> {
    (word(b, 0)? == SEND_RNDIS_PACKET).then(|| word(b, 4)).flatten()
}
