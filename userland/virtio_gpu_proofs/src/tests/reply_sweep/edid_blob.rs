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

//! EDIDs as a monitor might send them: a base block with four detailed
//! timings, and up to seven CTA-861 extensions of six more each, half of
//! them a size a driver can make and half at an edge or noise,
//! one block in eight with a checksum that does not add up.

use super::dtd::dtd;
use crate::tests::wire_sweep::rng::Rng;

fn base(r: &mut Rng, extensions: u8) -> [u8; 128] {
    let mut b = [0u8; 128];
    b[..8].copy_from_slice(&[0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0]);
    b[8..54].iter_mut().for_each(|x| *x = r.next() as u8);
    (b[18], b[19]) = (1, 3 + r.below(2) as u8);
    for slot in 0..4 {
        let (w, h) = size(r);
        b[54 + slot * 18..72 + slot * 18].copy_from_slice(&dtd(r, w, h));
    }
    b[126] = extensions;
    let sum = b[..127].iter().fold(0u8, |a, x| a.wrapping_add(*x));
    b[127] = if r.below(8) == 0 { sum } else { sum.wrapping_neg() };
    b
}

/// A CTA-861 extension: tag 2, revision 3, its detailed timings from byte 4.
fn cta(r: &mut Rng) -> [u8; 128] {
    let mut b = [0u8; 128];
    (b[0], b[1], b[2]) = (0x02, 3, 4);
    for slot in 0..6 {
        let (w, h) = size(r);
        b[4 + slot * 18..22 + slot * 18].copy_from_slice(&dtd(r, w, h));
    }
    let sum = b[..127].iter().fold(0u8, |a, x| a.wrapping_add(*x));
    b[127] = if r.below(8) == 0 { sum } else { sum.wrapping_neg() };
    b
}

/// A timing's size: a usual one half the time, else an edge or noise.
fn size(r: &mut Rng) -> (u32, u32) {
    if r.below(2) == 0 {
        (640 + r.below(1400), 480 + r.below(1000))
    } else {
        (r.word() & 0xFFF, r.word() & 0xFFF)
    }
}

/// A whole EDID: the base block, then its extensions.
pub fn blob(r: &mut Rng) -> Vec<u8> {
    let n = r.below(8) as u8;
    let mut out = base(r, n).to_vec();
    (0..n).for_each(|_| out.extend_from_slice(&cta(r)));
    out
}
