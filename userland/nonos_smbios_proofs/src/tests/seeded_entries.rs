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

//! Honest 32- and 64-bit entry points from a seed, with their sums set
//! right, for the F-105 sweep.

use super::seeded_gen::Rng;

pub fn fix(b: &mut [u8], span: core::ops::Range<usize>, at: usize) {
    b[at] = 0;
    let s = b[span].iter().fold(0u8, |s, x| s.wrapping_add(*x));
    b[at] = 0u8.wrapping_sub(s);
}

pub fn honest(r: &mut Rng, wide: bool) -> Vec<u8> {
    let mut b = r.bytes(if wide { 0x18 } else { 0x1F }, 0);
    if wide {
        b[..5].copy_from_slice(b"_SM3_");
        b[6] = 0x18;
        fix(&mut b, 0..0x18, 5);
    } else {
        b[..4].copy_from_slice(b"_SM_");
        b[5] = 0x1F;
        b[0x10..0x15].copy_from_slice(b"_DMI_");
        fix(&mut b, 0x10..0x1F, 0x15);
        fix(&mut b, 0..0x1F, 4);
    }
    b
}

pub fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |a, x| a << 8 | u64::from(*x))
}

/// One lie: a length byte or a byte from 8 on changed; half the time the
/// sums set right again over whatever the length now says (the "_DMI_"
/// part's only half of those); a quarter of the time, a cut.
pub fn lie(r: &mut Rng, b: &mut Vec<u8>, wide: bool) {
    let at = if r.below(3) == 0 { 5 + usize::from(wide) } else { 8 + r.below(23) as usize };
    if at < b.len() {
        b[at] = r.below(256) as u8;
    }
    if r.below(2) == 0 {
        let (sum, l) = if wide { (5, usize::from(b[6])) } else { (4, usize::from(b[5])) };
        if !wide && r.below(2) == 0 {
            fix(b, 0x10..0x1F, 0x15);
        }
        let end = l.clamp(sum + 1, b.len());
        fix(b, 0..end, sum);
    }
    if r.below(4) == 0 {
        b.truncate(b.len() - 1 - r.below(12) as usize);
    }
}
