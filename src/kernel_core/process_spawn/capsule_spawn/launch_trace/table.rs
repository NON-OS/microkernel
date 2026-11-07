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

use super::phase::Phase;
use super::record::Record;

/*
 * Launches that have queued a pid and not yet shown a frame. Small and
 * fixed: a launch that finds the table full pushes out the oldest, which
 * is written out as it stands rather than lost.
 */
pub(crate) struct Table<const N: usize> {
    slots: [Option<Record>; N],
    next: usize,
}

impl<const N: usize> Table<N> {
    pub(crate) const fn new() -> Self {
        Self { slots: [None; N], next: 0 }
    }

    pub(crate) fn len(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Keep `r`; returns the launch it displaced, if the table was full.
    pub(crate) fn insert(&mut self, r: Record) -> Option<Record> {
        if N == 0 {
            return Some(r);
        }
        if let Some(i) = self.find(r.pid()) {
            return self.slots[i].replace(r);
        }
        if let Some(free) = self.slots.iter_mut().find(|s| s.is_none()) {
            *free = Some(r);
            return None;
        }
        let i = self.next % N;
        self.next = (i + 1) % N;
        self.slots[i].replace(r)
    }

    /*
     * Marks a post-spawn phase for `pid`. The first frame completes a
     * launch: its record leaves the table and comes back to be written.
     */
    pub(crate) fn mark(&mut self, pid: u32, phase: Phase, now_ns: u64) -> Option<Record> {
        let i = self.find(pid)?;
        let slot = self.slots[i].as_mut()?;
        slot.mark(phase, now_ns);
        if phase == Phase::FirstFrame {
            return self.slots[i].take();
        }
        None
    }

    pub(crate) fn take(&mut self, pid: u32) -> Option<Record> {
        let i = self.find(pid)?;
        self.slots[i].take()
    }

    fn find(&self, pid: u32) -> Option<usize> {
        self.slots.iter().position(|s| matches!(s, Some(r) if r.pid() == pid))
    }
}
