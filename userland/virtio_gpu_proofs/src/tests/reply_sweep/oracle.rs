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

//! What the driver must make of a reply, from the virtio-gpu rules as the
//! driver's files state them: a reply counts only up to the length the
//! device claims, must be at least the command's whole answer (an EDID's
//! header and size word), must carry that command's OK type, and an EDID
//! must declare 128 to 1024 bytes, all within what was claimed, and open
//! with the base block's magic. The driver's buffer reads zero past what
//! the device wrote, as the staging page is cleared for each command.

use super::device::Reply;

pub const DISPLAY_ROOM: u32 = 24 + 24 * 16;
pub const EDID_ROOM: u32 = 24 + 8 + 1024;
const MAGIC: [u8; 8] = [0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0];

/// The buffer as the driver reads it, and the length the device claims.
pub fn seen(r: &Reply, room: u32) -> (Vec<u8>, u32) {
    let n = r.bytes.len().min(room as usize);
    let mut buf = r.bytes[..n].to_vec();
    buf.resize(room as usize, 0);
    (buf, r.claimed.unwrap_or(n as u32))
}

pub fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

/// Each scanout's (width, height, enabled), or None when refused.
pub fn display(r: &Reply) -> Option<Vec<(u32, u32, u32)>> {
    let (b, claimed) = seen(r, DISPLAY_ROOM);
    if claimed < DISPLAY_ROOM || word(&b, 0) != 0x1101 {
        return None;
    }
    Some(
        (0..16)
            .map(|i| {
                (word(&b, 24 + i * 24 + 8), word(&b, 24 + i * 24 + 12), word(&b, 24 + i * 24 + 16))
            })
            .collect(),
    )
}

/// The EDID blob, or None when refused.
pub fn edid(r: &Reply) -> Option<Vec<u8>> {
    let (b, claimed) = seen(r, EDID_ROOM);
    if claimed < 32 || word(&b, 0) != 0x1104 {
        return None;
    }
    let size = word(&b, 24) as usize;
    if !(128..=1024).contains(&size) || size > claimed as usize - 32 || b[32..40] != MAGIC {
        return None;
    }
    Some(b[32..32 + size].to_vec())
}
