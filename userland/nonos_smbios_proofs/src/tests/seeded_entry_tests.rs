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

//! F-105: seeded entry points. Honest 32- and 64-bit ones parse to exactly
//! what was written; then lengths that lie, contents changed with the sums
//! set right again, and cuts. Whatever parses has the length its form
//! needs, inside the bytes handed over, summing to zero (the "_DMI_" part
//! too), and its fields are the bytes at their offsets, nothing else.

use super::seeded_entries::{honest, le, lie};
use super::seeded_gen::Rng;
use crate::entry::parse_entry;

#[test]
fn entry_points_parse_only_as_their_bytes_say() {
    let (mut taken, mut refused) = (0, 0);
    for seed in 1..=6000u64 {
        let r = &mut Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let wide = r.below(2) == 0;
        let mut b = honest(r, wide);
        assert!(parse_entry(&b).is_some(), "seed {seed}: an honest entry");
        lie(r, &mut b, wide);
        let Some(e) = parse_entry(&b) else {
            refused += 1;
            continue;
        };
        taken += 1;
        let len = usize::from(b[5 + usize::from(wide)]);
        let sums = |s: &[u8]| s.iter().fold(0u8, |a, x| a.wrapping_add(*x)) == 0;
        assert!(len <= b.len() && sums(&b[..len]), "seed {seed}");
        if wide {
            assert!(len >= 0x18 && e.table == le(&b[16..24]) && u64::from(e.len) == le(&b[12..16]));
        } else {
            assert!(len >= 0x1F && &b[0x10..0x15] == b"_DMI_" && sums(&b[0x10..0x1F]));
            assert_eq!((e.table, u64::from(e.len)), (le(&b[0x18..0x1C]), le(&b[0x16..0x18])));
        }
    }
    assert!(taken > 1000 && refused > 1000, "both answers: {taken} {refused}");
}
