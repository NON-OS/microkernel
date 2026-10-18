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

//! HMAT tables from a seed: attributes, matrices whose domain counts run
//! past the cap or past the bytes, kinds the walk skips, lengths that lie.

use super::gen::{damage, sdt, Rng};

pub const MAX_DOMAINS: usize = 256; // either side of one matrix, as `hmat_decode` keeps

pub fn structure(ty: u16, rest: &[u8]) -> Vec<u8> {
    let len = (8 + rest.len()) as u32;
    [&ty.to_le_bytes()[..], &[0, 0], &len.to_le_bytes(), rest].concat()
}

fn matrix(r: &mut Rng) -> Vec<u8> {
    let m = r.pick(&[0, 1, 2, 3, MAX_DOMAINS, MAX_DOMAINS + 1]);
    let n = r.pick(&[0, 1, 2, 3, MAX_DOMAINS + 1]);
    let counts = [m as u32, n as u32, 0].map(u32::to_le_bytes).concat();
    [vec![r.next() as u8 & 3, r.below(7) as u8, 0, 0], counts, r.bytes(8 + 4 * (m + n) + 2 * m * n)]
        .concat()
}

pub fn hmat(r: &mut Rng) -> Vec<u8> {
    let mut body = vec![0; 4];
    for _ in 0..r.below(8) {
        let mut s = match r.below(5) {
            0 | 1 => structure(0, &r.bytes(32)),
            2 | 3 => structure(1, &matrix(r)),
            _ => {
                let (ty, n) = (r.next() as u16, r.below(24));
                structure(ty, &r.bytes(n))
            }
        };
        if r.one_in(8) {
            s[4] = r.next() as u8; // the length's low byte
        }
        body.extend(s);
    }
    damage(sdt(b"HMAT", 2, &body), r)
}
