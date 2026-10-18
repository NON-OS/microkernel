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

//! One address space's grant window, pure: ranges handed out first
//! fit from what earlier grants gave back, else from the untouched top, and
//! given back once their pages are unmapped. A long-running driver that maps
//! and unmaps no longer runs the window dry. kernel_proofs::va_window.

extern crate alloc;

use alloc::vec::Vec;

pub(crate) struct VaWindow {
    /// Below `top` every byte has been handed out once.
    top: u64,
    end: u64,
    /// Ranges given back, sorted and apart, each `[start, end)`.
    free: Vec<(u64, u64)>,
}

impl VaWindow {
    pub(crate) const fn new(base: u64, end: u64) -> Self {
        Self { top: base, end, free: Vec::new() }
    }

    /// `bytes` of window, or None when no range that long is left.
    pub(crate) fn reserve(&mut self, bytes: u64) -> Option<u64> {
        if bytes == 0 {
            return None;
        }
        if let Some(i) = self.free.iter().position(|&(s, e)| e - s >= bytes) {
            let (s, e) = self.free[i];
            if e - s == bytes {
                self.free.remove(i);
            } else {
                self.free[i].0 = s + bytes;
            }
            return Some(s);
        }
        let start = self.top;
        let next = start.checked_add(bytes).filter(|&n| n <= self.end)?;
        self.top = next;
        Some(start)
    }

    /// Give back `[start, start + bytes)`, a range `reserve` handed out.
    pub(crate) fn release(&mut self, start: u64, bytes: u64) {
        let end = start + bytes;
        let at = self.free.partition_point(|&(s, _)| s < start);
        self.free.insert(at, (start, end));
        if at + 1 < self.free.len() && self.free[at + 1].0 == end {
            self.free[at].1 = self.free.remove(at + 1).1;
        }
        if at > 0 && self.free[at - 1].1 == start {
            self.free[at - 1].1 = self.free.remove(at).1;
        }
        // What reaches the top lowers it.
        if self.free.last().is_some_and(|&(_, e)| e == self.top) {
            self.top = self.free.pop().map_or(self.top, |(s, _)| s);
        }
    }
}
