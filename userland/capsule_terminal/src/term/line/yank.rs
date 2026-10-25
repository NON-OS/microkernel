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

//! Putting the last cut text back at the cursor (Ctrl-Y).

use super::types::Line;

impl Line {
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
        // Whole characters, stopping at the first that no longer fits, so a
        // nearly full line takes what it can without splitting one.
        let text = match core::str::from_utf8(&buf[..n]) {
            Ok(t) => t,
            Err(e) => core::str::from_utf8(&buf[..e.valid_up_to()]).unwrap_or(""),
        };
        let mut any = false;
        for ch in text.chars() {
            if !self.insert_char(ch) {
                break;
            }
            any = true;
        }
        any
    }
}
