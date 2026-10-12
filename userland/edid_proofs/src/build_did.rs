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

//! DisplayID 2.0 sections and type VII descriptors.

use crate::build::seal;

/// A DisplayID 2.0 extension holding one section with `blocks` (each
/// with its three byte header already on), both checksums sealed.
pub fn displayid(blocks: &[u8]) -> [u8; 128] {
    displayid_v(0x20, blocks)
}

/// The same with the section's version byte `version` (0x13 for 1.3).
pub fn displayid_v(version: u8, blocks: &[u8]) -> [u8; 128] {
    let mut b = [0u8; 128];
    b[0] = 0x70;
    let s = &mut b[1..1 + 5 + blocks.len()];
    s[0] = version;
    s[1] = blocks.len() as u8;
    s[4..4 + blocks.len()].copy_from_slice(blocks);
    seal(s);
    seal(&mut b);
    b
}

/// A type VII descriptor (20 bytes), every field stored less one.
pub fn type7(clock_khz: u32, t: [u32; 8], options: u8) -> [u8; 20] {
    let mut d = [0u8; 20];
    d[..3].copy_from_slice(&(clock_khz - 1).to_le_bytes()[..3]);
    d[3] = options;
    for (i, v) in t.iter().enumerate() {
        let mut v = (v - 1) as u16;
        // Polarity bits ride on the front porch words; positive here.
        if i == 2 || i == 6 {
            v |= 0x8000;
        }
        d[4 + i * 2..6 + i * 2].copy_from_slice(&v.to_le_bytes());
    }
    d
}

/// A data block: tag, revision, payload length, then the payload.
pub fn block(tag: u8, rev: u8, body: &[u8]) -> Vec<u8> {
    let mut v = vec![tag, rev, body.len() as u8];
    v.extend_from_slice(body);
    v
}
