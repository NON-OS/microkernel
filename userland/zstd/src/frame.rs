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

//! A stream of frames: Zstandard frames decoded in turn, skippable frames
//! passed over. Each frame's content size and checksum are held to.

use alloc::vec::Vec;

use super::block::blocks;
use super::context::Context;
use super::frame_header::header;
use super::xxh64::xxh64;

const MAGIC: u32 = 0xFD2F_B528;
const SKIPPABLE: u32 = 0x184D_2A50;

pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let word = |at: usize| {
        let b = data.get(at..at.checked_add(4)?)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    let (mut out, mut at) = (Vec::new(), 0usize);
    if data.is_empty() {
        return None;
    }
    while at < data.len() {
        let magic = word(at)?;
        if magic & 0xFFFF_FFF0 == SKIPPABLE {
            let size = word(at + 4)? as usize;
            at = at.checked_add(8)?.checked_add(size)?;
            if at > data.len() {
                return None;
            }
            continue;
        }
        if magic != MAGIC {
            return None;
        }
        let h = header(data.get(at + 4..)?)?;
        at += 4 + h.used;
        let start = out.len();
        at += blocks(data.get(at..)?, &mut Context::new(), &mut out, start)?;
        if h.content.is_some_and(|n| n != out.len() - start) {
            return None;
        }
        if h.checksum {
            if word(at)? != xxh64(&out[start..], 0) as u32 {
                return None;
            }
            at += 4;
        }
    }
    Some(out)
}
