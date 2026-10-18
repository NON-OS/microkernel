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

//! Byte streams for the tests: uniform ones, and ones built to sit just
//! under or at a cutoff.

/// splitmix64's bytes: uniform enough that the tests' false alarm rate,
/// 2^-20 per sample, is all that could fail it, and fixed by `seed`.
pub(crate) fn uniform(seed: u64, n: usize) -> Vec<u8> {
    let mut s = seed;
    (0..n)
        .map(|_| {
            s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            (z ^ (z >> 31)) as u8
        })
        .collect()
}

/// One window opening on `a`, holding `a` `times` times in all, never twice
/// in a row, every other byte distinct from `a` and from its neighbours.
pub(crate) fn window_with(a: u8, times: usize, window: usize) -> Vec<u8> {
    let mut out = vec![a];
    let mut other = a;
    let mut next_other = || {
        other = other.wrapping_add(1);
        if other == a {
            other = other.wrapping_add(1);
        }
        other
    };
    for _ in 1..times {
        out.push(next_other());
        out.push(a);
    }
    while out.len() < window {
        out.push(next_other());
    }
    out
}
