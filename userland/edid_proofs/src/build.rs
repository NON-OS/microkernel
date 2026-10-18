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

//! EDIDs built byte by byte from the E-EDID 1.4 layout (section 3).

pub use crate::build_dtd::{display, dtd, dtd_1080p};

pub const HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

/// Make the block's bytes sum to zero through its last byte.
pub fn seal(b: &mut [u8]) {
    let n = b.len();
    b[n - 1] = 0;
    let s = b.iter().fold(0u8, |a, &v| a.wrapping_add(v));
    b[n - 1] = s.wrapping_neg();
}

/// A 1.4 base block, digital, 52 x 29 cm, no timings, all slots
/// dummy descriptors (tag 0x10), checksum sealed.
pub fn base() -> [u8; 128] {
    let mut b = [0u8; 128];
    b[..8].copy_from_slice(&HEADER);
    b[8..10].copy_from_slice(&0x30AEu16.to_be_bytes()); // "LEN"
    b[10..12].copy_from_slice(&0x40A1u16.to_le_bytes());
    b[12..16].copy_from_slice(&0x0102_0304u32.to_le_bytes());
    b[18] = 1;
    b[19] = 4;
    b[20] = 0xA5;
    b[21] = 52;
    b[22] = 29;
    b[38..54].fill(0x01);
    for i in 0..4 {
        b[54 + i * 18 + 3] = 0x10;
    }
    seal(&mut b);
    b
}

/// Put an 18 byte descriptor in base slot `slot` and reseal.
pub fn put(b: &mut [u8; 128], slot: usize, d: &[u8; 18]) {
    b[54 + slot * 18..54 + (slot + 1) * 18].copy_from_slice(d);
    seal(b);
}
