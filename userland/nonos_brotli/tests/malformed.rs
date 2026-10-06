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

/* Broken input must come back as an error, never a panic, a hang or
output past the cap: every cut of real streams, and mutated streams. */

mod common;

use common::inputs::Lcg;
use common::streams::cases;
use nonos_brotli::decompress;

#[test]
fn every_cut_is_refused() {
    for c in cases().iter().step_by(3) {
        let n = c.stream.len();
        let tail = n.saturating_sub(24)..n;
        for cut in (0..n).step_by(n / 120 + 1).chain(tail) {
            let got = decompress(&c.stream[..cut], 1 << 20);
            assert!(got.is_err(), "{} cut at {cut}", c.name());
        }
    }
}

#[test]
fn mutated_streams_stay_bounded() {
    let (all, mut g) = (cases(), Lcg(7));
    let mut outcomes = [0usize; 2];
    for _ in 0..30_000 {
        let c = &all[g.next() % all.len()];
        let mut s = c.stream.to_vec();
        if s.is_empty() {
            continue;
        }
        for _ in 0..1 + g.next() % 3 {
            let at = g.next() % s.len();
            match g.next() % 3 {
                0 => s[at] ^= 1 << (g.next() % 8),
                1 => s[at] = g.next() as u8,
                _ => s.insert(at, g.next() as u8),
            }
        }
        let cap = c.plain.len() + 4096;
        match decompress(&s, cap) {
            Ok(v) => {
                assert!(v.len() <= cap, "{} grew past the cap", c.name());
                outcomes[0] += 1;
            }
            Err(_) => outcomes[1] += 1,
        }
    }
    assert!(outcomes[1] > outcomes[0], "most mutations must be refused: {outcomes:?}");
}
