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

//! Exit statuses a kernel reader asked for before the process could end,
//! kept where no reaper reaches them. The reap log's entry is consumed by
//! the first parent that waits; a reader that is not the parent (init,
//! reading what setup asked for) must not lose its answer to that waiter.
//! Pure, with no kernel imports, so the host proofs mount it.

pub const KEPT_SLOTS: usize = 4;

pub struct Kept {
    slots: [(u32, Option<i32>); KEPT_SLOTS],
}

impl Kept {
    pub const fn new() -> Self {
        Self { slots: [(0, None); KEPT_SLOTS] }
    }

    /// Ask for `pid`'s status to be kept; false when every slot is taken.
    pub fn watch(&mut self, pid: u32) -> bool {
        if pid == 0 {
            return false;
        }
        if self.slots.iter().any(|s| s.0 == pid) {
            return true;
        }
        match self.slots.iter_mut().find(|s| s.0 == 0) {
            Some(slot) => {
                *slot = (pid, None);
                true
            }
            None => false,
        }
    }

    /// `pid` ended with `code`; kept only when it was asked for.
    pub fn note(&mut self, pid: u32, code: i32) {
        if let Some(slot) = self.slots.iter_mut().find(|s| s.0 == pid) {
            slot.1 = Some(code);
        }
    }

    /// The status kept for `pid`, read as often as asked; never one for pid 0,
    /// the free slots' pid.
    pub fn status(&self, pid: u32) -> Option<i32> {
        self.slots.iter().find(|s| pid != 0 && s.0 == pid).and_then(|s| s.1)
    }
}
