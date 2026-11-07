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

//! The changed rows of a frame told apart as runs: a command's output near
//! the Terminal's top and its status row at the bottom are two runs, not
//! the band of every row between them. At most MAX_PARTS runs, as many as
//! the compositor repaints apart (OP_PRESENT_LAYER_PARTS); runs fewer than
//! GAP_ROWS unchanged rows apart are one.

use crate::swap::MAX_PARTS;

/// Unchanged rows that keep two runs apart: a box for every few rows costs
/// the compositor more than the rows between.
pub const GAP_ROWS: usize = 16;

/// Runs of rows, each first..end, in order and apart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Runs {
    count: usize,
    runs: [(usize, usize); MAX_PARTS],
}

impl Runs {
    /// Run `r`, which starts at or past the end of every run before it.
    /// Full, the two neighbours with the fewest rows between, `r` among
    /// them, are joined.
    fn add(&mut self, r: (usize, usize)) {
        let Some(last) = self.count.checked_sub(1) else { return self.push(r) };
        let new_gap = r.0.saturating_sub(self.runs[last].1);
        if new_gap < GAP_ROWS {
            self.runs[last].1 = r.1;
            return;
        }
        if self.count < MAX_PARTS {
            return self.push(r);
        }
        let gap = |k: usize| self.runs[k].0.saturating_sub(self.runs[k - 1].1);
        match (1..self.count).min_by_key(|&k| gap(k)) {
            Some(k) if gap(k) < new_gap => {
                self.join(k);
                self.push(r);
            }
            _ => self.runs[last].1 = r.1,
        }
    }

    /// `r` after the last run; there is room.
    fn push(&mut self, r: (usize, usize)) {
        self.runs[self.count] = r;
        self.count += 1;
    }

    /// Run `k` joined to the one before it.
    fn join(&mut self, k: usize) {
        self.runs[k - 1].1 = self.runs[k].1;
        self.runs.copy_within(k + 1..self.count, k);
        self.count -= 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.runs[..self.count].iter().copied()
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

/// The runs of rows in `span` (first..end) where `next` and `shown` differ,
/// rows `row_words` pixels apart. Rows outside `span`, or past either
/// frame, are not read.
pub fn runs_in(
    next: &[u32],
    shown: &[u32],
    row_words: usize,
    (first, end): (usize, usize),
) -> Runs {
    let mut runs = Runs::default();
    if row_words == 0 {
        return runs;
    }
    let rows = next.chunks_exact(row_words).zip(shown.chunks_exact(row_words));
    let mut open = None;
    for (y, (a, b)) in rows.enumerate().take(end).skip(first) {
        match (a != b, open) {
            (true, None) => open = Some(y),
            (false, Some(s)) => {
                runs.add((s, y));
                open = None;
            }
            _ => {}
        }
    }
    if let Some(s) = open {
        runs.add((s, end));
    }
    runs
}
