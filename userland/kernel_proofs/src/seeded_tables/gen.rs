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

//! Seeded hostile tables: structures whose length bytes
//! lie or are cut short, and whole tables whose declared length runs past
//! or stops short of the bytes handed over, resealed so the header checks
//! pass and the parser meets the damage itself.

pub use crate::arm_acpi_tests::build::{sdt, seal, structure as honest};
pub use super::rng::Rng;

/// A structure: `ty`, an 8-bit length, `body` random bytes after them; the
/// length byte sometimes lies and the bytes are sometimes cut.
pub fn structure(r: &mut Rng, ty: u8, body: usize) -> Vec<u8> {
    let mut s = [vec![ty, (body + 2) as u8], r.bytes(body)].concat();
    if r.one_in(8) {
        s[1] = r.next() as u8;
    }
    if r.one_in(10) {
        s.truncate(r.below(s.len()));
    }
    s
}

/// The checksum byte set so `t[..upto]` sums to zero.
fn fix_sum(t: &mut [u8], upto: usize) {
    let upto = upto.min(t.len());
    if upto > 9 {
        t[9] = 0;
        let sum = t[..upto].iter().fold(0u8, |s, b| s.wrapping_add(*b));
        t[9] = 0u8.wrapping_sub(sum);
    }
}

/// Sealed (every table built here is at least a header), then left whole,
/// cut short, given a declared length past or short of its bytes, or a few
/// bytes changed with the sum mostly set right again.
pub fn damage(mut t: Vec<u8>, r: &mut Rng) -> Vec<u8> {
    seal(&mut t);
    match r.below(6) {
        0 | 1 => {}
        2 => t.truncate(r.below(t.len() + 1)),
        3 | 4 => {
            let any = r.next() as usize;
            let lie = r.pick(&[0, 1, 35, 36, 44, t.len() - 1, t.len() + 1, any]);
            t[4..8].copy_from_slice(&(lie as u32).to_le_bytes());
            fix_sum(&mut t, lie);
        }
        _ => {
            for _ in 0..1 + r.below(4) {
                let at = 8 + r.below(t.len() - 8);
                t[at] = r.next() as u8;
            }
            // one time in four the sum is left broken
            if !r.one_in(4) {
                seal(&mut t);
            }
        }
    }
    t
}
