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

//! What the kill keys cut, so it can be put back.
//!
//! Ctrl-U, Ctrl-W and Ctrl-K are not delete. They are cut, and every shell
//! anyone arrives here from pairs them with Ctrl-Y to paste the last one
//! back. Without the ring they are the same key as a long press of
//! backspace, and the moment a whole typed line goes away with no way to
//! recover it is the moment a terminal stops feeling like one.
//!
//! One slot rather than a stack of them. The recovery people actually reach
//! for is the last thing they cut, and a ring you have to walk needs a key to
//! walk it with, which is a second thing to learn for a case that comes up
//! rarely.

use super::types::Line;

impl Line {
    /// Remember `bytes` as the last thing cut.
    ///
    /// Cutting nothing leaves the previous contents alone: a Ctrl-U on an
    /// already empty line should not destroy what a Ctrl-U a moment ago put
    /// there, which is exactly when someone is about to press Ctrl-Y.
    pub(super) fn hold_killed(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        let n = bytes.len().min(self.killed.len());
        self.killed[..n].copy_from_slice(&bytes[..n]);
        self.killed_len = n;
    }

    /// Cut the whole line into the ring and empty it (Ctrl-U).
    ///
    /// Separate from `clear`, which is what Ctrl-C and the return key use.
    /// Those abandon a line rather than cut it, and a ring filled by every
    /// command that ever ran is a ring holding something nobody chose.
    pub fn kill_line(&mut self) -> bool {
        if self.len == 0 {
            self.cursor = 0;
            return false;
        }
        let mut cut = [0u8; super::types::KILL_CAP];
        let n = self.len;
        cut[..n].copy_from_slice(&self.buf[..n]);
        self.hold_killed(&cut[..n]);
        self.clear();
        true
    }

    /// Insert the last cut text at the cursor (Ctrl-Y).
    ///
    /// The text stays in the ring after being pasted, so it can be put in
    /// more than one place, which is half of why the ring is worth having.
    pub fn yank(&mut self) -> bool {
        if self.killed_len == 0 {
            return false;
        }
        let mut buf = [0u8; super::types::KILL_CAP];
        let n = self.killed_len;
        buf[..n].copy_from_slice(&self.killed[..n]);
        let mut any = false;
        for &b in &buf[..n] {
            any |= self.insert(b);
        }
        any
    }
}
