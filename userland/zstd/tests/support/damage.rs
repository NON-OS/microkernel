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

//! The ways hostile bytes arrive: flips, overwrites, truncation, insertion,
//! lengths set to all ones, and spans cut out.

use crate::rng::Rng;

pub fn mutate(r: &mut Rng, v: &mut Vec<u8>) {
    let pick = |r: &mut Rng, n: usize| (r.next() % n.max(1) as u64) as usize;
    for _ in 0..=pick(r, 4) {
        let at = pick(r, v.len());
        match r.next() % 6 {
            0 if !v.is_empty() => v[at] ^= 1 << (r.next() % 8),
            1 if !v.is_empty() => v[at] = (r.next() >> 56) as u8,
            2 => v.truncate(at),
            3 => v.insert(at.min(v.len()), (r.next() >> 56) as u8),
            4 if at + 4 <= v.len() => v[at..at + 4].copy_from_slice(&[0xFF; 4]),
            _ if at < v.len() => {
                let len = pick(r, 16).min(v.len() - at);
                v.drain(at..at + len);
            }
            _ => {}
        }
    }
}
