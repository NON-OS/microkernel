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

//! Coverage at pixel granularity for the damage proofs: the cells of a
//! rectangle, and the check that every damaged cell is drained again.

use crate::damage::{DamageAccumulator, Rect};
use std::collections::HashSet;

pub(crate) fn cells(r: Rect) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    for y in r.y..r.y + r.height {
        for x in r.x..r.x + r.width {
            v.push((x, y));
        }
    }
    v
}

// Accumulate every input, drain everything, and assert the drained rectangles
// cover every cell of every input.
pub(crate) fn assert_covers(inputs: &[Rect]) {
    let mut acc = DamageAccumulator::new();
    let mut wanted: HashSet<(u32, u32)> = HashSet::new();
    for &r in inputs {
        if r.width != 0 && r.height != 0 {
            wanted.extend(cells(r));
        }
        acc.accumulate(r);
    }
    let mut covered: HashSet<(u32, u32)> = HashSet::new();
    while let Some(out) = acc.drain() {
        covered.extend(cells(out));
    }
    let missing: Vec<_> = wanted.difference(&covered).collect();
    assert!(missing.is_empty(), "uncovered cells: {:?}", missing);
}
