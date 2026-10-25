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

//! Seeded EDIDs for the sweep: one to six blocks, each sealed with its
//! checksum; a base that declares the truth or an edge (none, too many,
//! 255); sometimes an HF-EEODB in block 1 naming an edge count; and now
//! and then a block the sink itself holds broken.

use super::own::{BLOCK, HEADER};
use crate::tests::wire_sweep::rng::Rng;

const COUNTS: [u8; 7] = [0, 1, 2, 3, 4, 5, 255];

pub fn edid(r: &mut Rng) -> Vec<u8> {
    let present = 1 + r.below(6) as usize;
    let mut e: Vec<u8> = (0..present * BLOCK).map(|_| r.next() as u8).collect();
    e[..8].copy_from_slice(&HEADER);
    e[126] = match r.below(3) {
        0 => COUNTS[r.below(7) as usize],
        _ => (present - 1) as u8,
    };
    if present > 1 && r.below(2) == 0 {
        cta(&mut e[BLOCK..2 * BLOCK], r);
        e[126] = if r.below(2) == 0 { 1 } else { e[126] };
    }
    e.chunks_mut(BLOCK).for_each(seal);
    if r.below(10) == 0 {
        let k = r.below(present as u32) as usize;
        e[k * BLOCK + 127] ^= 1 << r.below(8);
    }
    e
}

/// A CTA block whose first data block is an HF-EEODB, now and then off
/// by one in where its collection ends or in its extended tag.
fn cta(b: &mut [u8], r: &mut Rng) {
    (b[0], b[1], b[3], b[4]) = (2, 3, 0, 0xE2);
    b[2] = [7, 7, 7, 6, 8, 0][r.below(6) as usize];
    b[5] = if r.below(8) == 0 { 0x77 } else { 0x78 };
    b[6] = COUNTS[r.below(7) as usize];
}

fn seal(b: &mut [u8]) {
    b[127] = 0;
    b[127] = b.iter().fold(0u8, |a, v| a.wrapping_add(*v)).wrapping_neg();
}
