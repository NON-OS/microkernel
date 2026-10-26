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

//! Free RAM as sorted, disjoint `[start, end)` runs, in fixed storage since
//! it is built before the heap. Pure, so it is held on the host
//! (boot_memory_proofs).

/// Runs kept; when there are more, the smallest are left out, which costs
/// RAM but never hands out memory that is not free.
pub(crate) const MAX_RUNS: usize = 32;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Usable {
    runs: [(u64, u64); MAX_RUNS],
    n: usize,
}

impl Usable {
    pub(crate) const EMPTY: Self = Self { runs: [(0, 0); MAX_RUNS], n: 0 };

    pub(crate) fn runs(&self) -> &[(u64, u64)] {
        &self.runs[..self.n]
    }

    pub(crate) fn largest(&self) -> Option<(u64, u64)> {
        self.runs().iter().copied().max_by_key(|&(a, b)| b - a)
    }

    /// Add `[start, end)`, merged with every run it meets or touches.
    pub(crate) fn add(&mut self, start: u64, end: u64) {
        let (mut s, mut e) = (start, end);
        if e <= s {
            return;
        }
        let mut i = 0;
        while i < self.n {
            let (a, b) = self.runs[i];
            if a <= e && s <= b {
                (s, e) = (s.min(a), e.max(b));
                self.runs.copy_within(i + 1..self.n, i);
                self.n -= 1;
            } else {
                i += 1;
            }
        }
        if self.n == MAX_RUNS {
            let Some(k) = (0..self.n).min_by_key(|&k| self.runs[k].1 - self.runs[k].0) else {
                return;
            };
            if self.runs[k].1 - self.runs[k].0 >= e - s {
                return;
            }
            self.runs.copy_within(k + 1..self.n, k);
            self.n -= 1;
        }
        let at = self.runs().iter().position(|r| r.0 > s).unwrap_or(self.n);
        self.runs.copy_within(at..self.n, at + 1);
        self.runs[at] = (s, e);
        self.n += 1;
    }
}
