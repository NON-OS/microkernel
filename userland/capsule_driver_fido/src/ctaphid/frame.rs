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
//! CTAPHID framing (FIDO CTAP 2.x, 11.2.4, as recalled, unverified): a
//! message goes in 64-byte reports. The first is the channel (4 bytes),
//! the command with bit 7 set, the length (big-endian u16) and 57 bytes of
//! data; each after it is the channel, a sequence number 0 to 127 and 59
//! bytes more.

use alloc::vec::Vec;

pub const PACKET: usize = 64;
const INIT_DATA: usize = PACKET - 7;
const CONT_DATA: usize = PACKET - 5;
/// The longest message the framing carries: 57 + 128 * 59 bytes.
pub const MESSAGE_MAX: usize = INIT_DATA + 128 * CONT_DATA;

pub type Cid = [u8; 4];
/// The channel every application may use, for INIT alone.
pub const BROADCAST: Cid = [0xFF; 4];

pub const CMD_INIT: u8 = 0x86;
pub const CMD_CBOR: u8 = 0x90;
pub const CMD_CANCEL: u8 = 0x91;
pub const CMD_KEEPALIVE: u8 = 0xBB;
pub const CMD_ERROR: u8 = 0xBF;

/// `data` as the reports of one `cmd` on `cid`; `None` past MESSAGE_MAX.
pub fn packets(cid: Cid, cmd: u8, data: &[u8]) -> Option<Vec<[u8; PACKET]>> {
    if data.len() > MESSAGE_MAX {
        return None;
    }
    let mut first = [0u8; PACKET];
    first[..4].copy_from_slice(&cid);
    first[4] = cmd;
    first[5..7].copy_from_slice(&(data.len() as u16).to_be_bytes());
    let head = data.len().min(INIT_DATA);
    first[7..7 + head].copy_from_slice(&data[..head]);
    let mut out = alloc::vec![first];
    for (seq, chunk) in data[head..].chunks(CONT_DATA).enumerate() {
        let mut p = [0u8; PACKET];
        p[..4].copy_from_slice(&cid);
        p[4] = seq as u8;
        p[5..5 + chunk.len()].copy_from_slice(chunk);
        out.push(p);
    }
    Some(out)
}
