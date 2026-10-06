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

//! ED and EL. Erasing never moves the cursor, and erased cells keep the
//! pen's background, as xterm erases.

use super::state::Term;

impl Term {
    pub(super) fn erase_in_line(&mut self, mode: u16) {
        let cols = self.cols;
        let scr = self.scr();
        let (x, y) = (scr.cur.x, scr.cur.y);
        let blank = scr.blank();
        let line = &mut scr.lines[y];
        match mode {
            0 => {
                line.erase(x, cols, blank);
                line.wrapped = false;
            }
            1 => line.erase(0, x + 1, blank),
            2 => line.clear(blank),
            _ => return,
        }
        self.touch(y);
    }

    pub(super) fn erase_display(&mut self, mode: u16) {
        let (cols, rows) = (self.cols, self.rows);
        let scr = self.scr();
        let (x, y) = (scr.cur.x, scr.cur.y);
        let blank = scr.blank();
        match mode {
            0 => {
                scr.lines[y].erase(x, cols, blank);
                scr.lines[y].wrapped = false;
                for line in &mut scr.lines[y + 1..rows] {
                    line.clear(blank);
                }
            }
            1 => {
                for line in &mut scr.lines[..y] {
                    line.clear(blank);
                }
                scr.lines[y].erase(0, x + 1, blank);
            }
            2 => {
                for line in scr.lines.iter_mut() {
                    line.clear(blank);
                }
            }
            3 => {
                self.clear_history();
                return;
            }
            _ => return,
        }
        self.touch_all();
    }
}
