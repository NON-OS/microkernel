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

//! The desk asserted on: no composed pixel FOREIGN, the screen equal to a
//! full frame, and the pointer's staircase across the screen checking both.

use super::foreign_desk::*;
use super::foreign_kernel::*;

impl Desk {
    pub(crate) fn assert_clean(&mut self, what: &str) {
        if let Some(i) = self.canvas.iter().position(|&p| p == FOREIGN) {
            panic!(
                "{what}: pixel ({}, {}) came from another process's memory",
                i as u32 % W,
                i as u32 / W
            );
        }
    }

    pub(crate) fn assert_current(&mut self, what: &str) {
        let want = self.reference();
        if let Some(i) = (0..want.len()).find(|&i| self.canvas[i] != want[i]) {
            let (x, y) = (i as u32 % W, i as u32 / W);
            panic!(
                "{what}: ({x}, {y}) shows {:#010x}, a full frame {:#010x}",
                self.canvas[i], want[i]
            );
        }
    }

    /// The pointer crosses the screen in a staircase, a frame each step.
    pub(crate) fn staircase(&mut self, what: &str) {
        for step in 0..12 {
            self.move_cursor((8 + step * 6).min(W - 1), (4 + step * 5).min(H - 1));
            self.frame();
            self.assert_clean(what);
        }
    }
}
