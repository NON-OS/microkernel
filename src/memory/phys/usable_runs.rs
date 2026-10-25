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

//! The boot map's usable RAM as merged runs (L60), so a firmware region a
//! device is to reach for good can be checked against memory the allocator
//! hands out. Past the room kept, every question answers "touches": a region
//! is refused rather than guessed. Pure, so the host proofs hold it.

pub(crate) const ROOM: usize = 128;

/// Merged `[start, end)` runs, sorted, and whether one was left out.
pub(crate) struct Runs {
    pub runs: [(u64, u64); ROOM],
    pub len: usize,
    pub full: bool,
}

impl Runs {
    pub(crate) const EMPTY: Runs = Runs { runs: [(0, 0); ROOM], len: 0, full: false };

    /// Add `[start, end)` from a list sorted by start, merged with the last
    /// run when they touch or overlap.
    pub(crate) fn add(&mut self, start: u64, end: u64) {
        if end <= start {
            return;
        }
        if let Some(last) = self.runs[..self.len].last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                return;
            }
        }
        match self.runs.get_mut(self.len) {
            Some(slot) => {
                *slot = (start, end);
                self.len += 1;
            }
            None => self.full = true,
        }
    }

    /// Whether `[start, end)` shares a byte with usable RAM, or might.
    pub(crate) fn touches(&self, start: u64, end: u64) -> bool {
        self.full || self.runs[..self.len].iter().any(|&(s, e)| s < end && start < e)
    }
}
