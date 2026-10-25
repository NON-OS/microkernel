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

//! The test's own reading of what it holds the driver to, from the layouts
//! as this test reads them (VESA E-EDID 1.4 sections 3.3 and 3.11, HDMI
//! 2.1's HF-EEODB, DDC/CI's reply framing), written apart from the crates.

pub const BLOCK: usize = 128;
pub const HEADER: [u8; 8] = [0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0];

pub fn sums(b: &[u8]) -> bool {
    b.iter().fold(0u8, |a, v| a.wrapping_add(*v)) == 0
}

/// Block 1's HF-EEODB count: a CTA block (tag 2) of revision 3 or later
/// whose first data block, inside its data block collection, is the
/// extended tag 0x78 with a count of at least one.
pub fn eeodb(b1: &[u8]) -> Option<usize> {
    if b1.len() != BLOCK || !sums(b1) || b1[0] != 2 || b1[1] < 3 {
        return None;
    }
    let (d, h) = (b1[2] as usize, b1[4] as usize);
    let n = h & 0x1F;
    let inside = (4..=127).contains(&d) && 5 + n <= d;
    (inside && h >> 5 == 7 && n >= 2 && b1[5] == 0x78 && b1[6] >= 1).then_some(b1[6] as usize)
}

/// The extensions the kept blocks declare.
pub fn declared(kept: &[u8]) -> usize {
    match kept.len() / BLOCK {
        0 => 0,
        1 => kept[126] as usize,
        _ => eeodb(&kept[BLOCK..2 * BLOCK]).unwrap_or(kept[126] as usize),
    }
}

/// A block worth keeping after `kept`: it sums to zero; the base starts
/// with the header, and an extension repeats no block before it.
pub fn whole(b: &[u8], kept: &[u8]) -> bool {
    let fresh = match kept.is_empty() {
        true => b[..8] == HEADER,
        false => !kept.chunks(BLOCK).any(|p| p == b),
    };
    b.len() == BLOCK && sums(b) && fresh
}
