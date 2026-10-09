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

//! Tab stops: every eighth column until a program sets its own.

use alloc::vec::Vec;

use super::state::Term;

pub(super) fn default_tabs(cols: usize) -> Vec<bool> {
    (0..cols).map(|x| x > 0 && x % 8 == 0).collect()
}

impl Term {
    pub(super) fn tab_forward(&mut self, n: usize) {
        let last = self.cols - 1;
        let mut x = self.scr_ref().cur.x;
        for _ in 0..n.min(self.cols) {
            match (x + 1..self.cols).find(|&c| self.tabs[c]) {
                Some(next) => x = next,
                None => {
                    x = last;
                    break;
                }
            }
        }
        let cur = &mut self.scr().cur;
        cur.x = x;
        cur.pending_wrap = false;
    }

    pub(super) fn tab_back(&mut self, n: usize) {
        let mut x = self.scr_ref().cur.x;
        for _ in 0..n.min(self.cols) {
            match (0..x).rev().find(|&c| self.tabs[c]) {
                Some(prev) => x = prev,
                None => {
                    x = 0;
                    break;
                }
            }
        }
        let cur = &mut self.scr().cur;
        cur.x = x;
        cur.pending_wrap = false;
    }
}
