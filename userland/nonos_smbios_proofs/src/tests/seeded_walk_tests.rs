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

//! F-105: the walk over seeded tables with one lie each. Every honest
//! structure before the lie is found, exactly as the model has it, and
//! nothing after a lie that stops the walk; a count from the entry point is
//! honoured; no slice the walk hands back lies outside the copied table;
//! and a string is found only for an index within its set.

use super::seeded_gen::Rng;
use super::seeded_lies::{table, Lie};
use crate::identity::identity;
use crate::walk::structures;

/// Whether `s` lies inside `t`, as a borrow of it.
fn within(t: &[u8], s: &[u8]) -> bool {
    let (a, b) = (t.as_ptr() as usize, s.as_ptr() as usize);
    s.is_empty() || (a <= b && b + s.len() <= a + t.len())
}

#[test]
fn honest_structures_before_a_lie_are_found_and_nothing_past_the_table_is_read() {
    let mut seen = [0u32; 5];
    for seed in 1..=4000u64 {
        let r = &mut Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let (t, want, lie) = table(r);
        seen[lie as usize] += 1;
        let count = (r.below(4) == 0).then(|| r.below(16) as u16);
        let got = structures(&t, count);
        let _ = identity(&got); // read from whatever the lie left, never past it
        let n = count.map_or(want.len(), |c| want.len().min(usize::from(c)));
        assert_eq!(got.len(), n, "seed {seed}: {lie:?}");
        for (g, w) in got.iter().zip(&want) {
            assert_eq!((g.kind, g.handle, &g.formatted[4..]), (w.kind, w.handle, &w.fields[..]));
            assert!(g.strings.iter().map(|s| s.to_vec()).eq(w.strings.iter().cloned()), "{seed}");
            assert!(within(&t, g.formatted) && g.strings.iter().all(|s| within(&t, s)));
            for off in 0..g.formatted.len() + 4 {
                let index = g.byte(off).map(usize::from).unwrap_or(0);
                let named = (1..=g.strings.len()).contains(&index);
                assert_eq!(g.string(off).is_some(), named, "seed {seed} offset {off}");
            }
        }
    }
    assert!(seen.iter().all(|c| *c > 500), "every lie met: {seen:?} ({:?})", Lie::None);
}
