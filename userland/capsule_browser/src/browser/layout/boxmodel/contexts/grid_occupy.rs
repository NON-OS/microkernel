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

use alloc::vec::Vec;

/* Tracks on the axis items fill across (columns, in row flow): one bit
 * each in a row's occupancy word. More are laid in the last one. */
pub(in super::super) const MAX_COLS: usize = 64;
/* Tracks on the axis that grows (rows, in row flow), so a hostile
 * placement cannot grow the table without bound; items past it stack in
 * the last. */
pub(in super::super) const MAX_ROWS: usize = 512;
/* The longest span a placement may ask for. */
pub(in super::super) const MAX_SPAN: usize = MAX_ROWS;

/* One grid area: first row and column (0-based) and the spans. */
#[derive(Clone, Copy, Default)]
pub(in super::super) struct Area {
    pub r: usize,
    pub c: usize,
    pub rs: usize,
    pub cs: usize,
}

/* Which cells hold an item: a bit per column in a word per row. */
pub(in super::super) struct Occupy {
    rows: Vec<u64>,
}

fn mask(c: usize, cs: usize) -> u64 {
    let bits = if cs >= 64 { u64::MAX } else { (1u64 << cs) - 1 };
    bits.checked_shl(c as u32).unwrap_or(0)
}

impl Occupy {
    pub(in super::super) fn new() -> Self {
        Occupy { rows: Vec::new() }
    }

    /* Rows that hold anything. */
    pub(in super::super) fn rows(&self) -> usize {
        self.rows.len()
    }

    pub(in super::super) fn fits(&self, a: Area) -> bool {
        let m = mask(a.c, a.cs);
        (a.r..(a.r + a.rs).min(MAX_ROWS)).all(|r| self.rows.get(r).is_none_or(|w| w & m == 0))
    }

    pub(in super::super) fn mark(&mut self, a: Area) {
        let r = a.r.min(MAX_ROWS - 1);
        let end = (r + a.rs).clamp(r + 1, MAX_ROWS);
        if self.rows.len() < end {
            self.rows.resize(end, 0);
        }
        let m = mask(a.c, a.cs);
        self.rows.iter_mut().take(end).skip(r).for_each(|w| *w |= m);
    }
}
