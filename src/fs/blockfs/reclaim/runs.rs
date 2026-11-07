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

//! Sets of block numbers as runs, kept small because files' blocks are
//! mostly given out in order.

use alloc::vec::Vec;

/// Half-open runs [start, end), as they were added.
#[derive(Default)]
pub(crate) struct Runs(pub Vec<(u64, u64)>);

impl Runs {
    pub(crate) fn add(&mut self, lba: u64) {
        if let Some(last) = self.0.last_mut() {
            if last.1 == lba {
                last.1 += 1;
                return;
            }
            if last.0 <= lba && lba < last.1 {
                return;
            }
        }
        self.0.push((lba, lba + 1));
    }

    /// Sorted and merged.
    pub(crate) fn settle(mut self) -> Vec<(u64, u64)> {
        self.0.sort_unstable();
        let mut out: Vec<(u64, u64)> = Vec::with_capacity(self.0.len());
        for (s, e) in self.0 {
            match out.last_mut() {
                Some(last) if s <= last.1 => last.1 = last.1.max(e),
                _ => out.push((s, e)),
            }
        }
        out
    }
}

/// What of [lo, hi) the sorted, merged `taken` does not cover.
pub(crate) fn gaps(lo: u64, hi: u64, taken: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    let mut at = lo;
    for &(s, e) in taken {
        if e <= at {
            continue;
        }
        if s >= hi {
            break;
        }
        if s > at {
            out.push((at, s.min(hi)));
        }
        at = at.max(e);
    }
    if at < hi {
        out.push((at, hi));
    }
    out
}
