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

//! Up to three mutations of a well-formed reply, each at an edge of what
//! the driver checks (its type, its length, the length the device claims,
//! an EDID's size word) or plain noise.

use super::device::Reply;
use crate::tests::wire_sweep::rng::Rng;

/// The OK types and one refusal, so a type swap often lands on another
/// command's success.
const TYPES: [u32; 6] = [0x1100, 0x1101, 0x1102, 0x1104, 0x1200, 0x1203];

/// `good` with up to three mutations; `room` is the buffer the driver offers.
pub fn mutate(r: &mut Rng, mut bytes: Vec<u8>, room: u32) -> Reply {
    let mut claimed = None;
    for _ in 0..r.below(4) {
        match r.below(6) {
            0 if bytes.len() >= 4 => {
                bytes[..4].copy_from_slice(&TYPES[r.below(6) as usize].to_le_bytes())
            }
            1 => bytes.truncate(r.below(bytes.len() as u32 + 1) as usize),
            2 => claimed = Some(edge(r, bytes.len() as u32, room)),
            3 if bytes.len() >= 28 => {
                bytes[24..28].copy_from_slice(&edge(r, 0, 1024).to_le_bytes())
            }
            4 if !bytes.is_empty() => {
                let at = r.below(bytes.len() as u32) as usize;
                bytes[at] ^= 1 << r.below(8);
            }
            _ => bytes.extend((0..r.below(64)).map(|_| r.next() as u8)),
        }
    }
    Reply { bytes, claimed }
}

/// A length at an edge of `len` or `room`, or noise.
fn edge(r: &mut Rng, len: u32, room: u32) -> u32 {
    let picks = [
        0,
        1,
        23,
        24,
        31,
        32,
        160,
        len.saturating_sub(1),
        len,
        len + 1,
        room - 1,
        room,
        room + 1,
        u32::MAX,
    ];
    if r.below(8) == 0 {
        r.next() as u32
    } else {
        picks[r.below(picks.len() as u32) as usize]
    }
}
