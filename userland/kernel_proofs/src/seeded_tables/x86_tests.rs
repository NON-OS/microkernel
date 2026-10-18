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

//! The x86 MADT's structure walk: lengths that lie both in the header and
//! in each structure, and a flood past the cap, which an undamaged table
//! must stop at exactly.

use super::gen::{damage, honest, sdt};
use super::madt_walk::{structures, MAX_STRUCTURES};
use super::sweep::{body, held, sweep};

const MADT: [(u8, usize); 7] = [(0, 6), (1, 10), (2, 8), (4, 4), (5, 10), (9, 14), (10, 10)];

#[test]
fn madt_structures_stay_inside_the_table() {
    let lens = |t: &[u8]| -> Vec<(usize, usize)> {
        structures(t).map(|s| (usize::from(s[1]), s.len())).collect()
    };
    sweep(
        |r| damage(sdt(b"APIC", 5, &body(r, 8, &MADT)), r),
        |t| Some(lens(t)).filter(|l| !l.is_empty()),
        |l, t| {
            let whole = l.iter().all(|(said, len)| said == len && *len >= 2);
            whole
                && l.len() <= MAX_STRUCTURES
                && 44 + l.iter().map(|l| l.1).sum::<usize>() <= held(t)
        },
    );
}

#[test]
fn madt_walk_stops_at_its_cap() {
    let t = sdt(b"APIC", 5, &[vec![0; 8], honest(0x7F, &[]).repeat(MAX_STRUCTURES + 3)].concat());
    assert_eq!(structures(&t).count(), MAX_STRUCTURES);
}
