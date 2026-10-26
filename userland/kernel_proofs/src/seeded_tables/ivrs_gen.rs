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
//! IVRS's blocks for the sweep (AMD IOMMU specification 48882, section 5.2):
//! IVHD 10h with its 24-byte header, 11h and 40h with 40, each with device
//! entries of four, eight or variable size, and IVMD 20h to 22h. A block's
//! length sometimes lies and a block is sometimes cut; IVMD ranges are
//! mostly sound, so the parser is seen to keep some.

use super::gen::Rng;
use super::ivrs_entry::entry;

fn ivhd(r: &mut Rng, kind: u8) -> Vec<u8> {
    let mut b = r.bytes(if kind == 0x10 { 20 } else { 36 });
    if !r.one_in(8) {
        b[4..12].copy_from_slice(&((1 + r.below(6) as u64) << 20).to_le_bytes());
    }
    if r.one_in(8) {
        // Shorter than any IVHD header, its base still inside it.
        b.truncate(12 + r.below(8));
        return b;
    }
    (0..r.below(16)).for_each(|_| b.extend(entry(r)));
    b
}

fn ivmd(r: &mut Rng) -> Vec<u8> {
    let mut b = r.bytes(28);
    if !r.one_in(4) {
        let (start, len) = ((r.below(1 << 20) as u64) << 12, (1 + r.below(64) as u64) << 12);
        b[12..28].copy_from_slice(&[start.to_le_bytes(), len.to_le_bytes()].concat());
    }
    b
}

/// One block, one time in eight of a kind IVRS does not define.
pub fn block(r: &mut Rng) -> Vec<u8> {
    let kind =
        if r.one_in(8) { r.next() as u8 } else { r.pick(&[0x10, 0x11, 0x40, 0x20, 0x21, 0x22]) };
    let body = match kind {
        0x10 | 0x11 | 0x40 => ivhd(r, kind),
        0x20..=0x22 => ivmd(r),
        _ => r.some_bytes(40),
    };
    let length = if r.one_in(10) { r.next() as u16 } else { (body.len() + 4) as u16 };
    let mut b = [vec![kind, r.next() as u8], length.to_le_bytes().to_vec(), body].concat();
    if r.one_in(12) {
        b.truncate(r.below(b.len()));
    }
    b
}

/// An IVRS body: IVinfo, eight reserved, then blocks.
pub fn ivrs_body(r: &mut Rng) -> Vec<u8> {
    let mut b = [r.bytes(4), vec![0; 8]].concat();
    (0..r.below(10)).for_each(|_| b.extend(block(r)));
    b
}
