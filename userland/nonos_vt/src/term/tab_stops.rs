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

//! HTS and TBC: setting and clearing tab stops.

use super::state::Term;

impl Term {
    pub(super) fn set_tab(&mut self) {
        let x = self.scr_ref().cur.x;
        if let Some(t) = self.tabs.get_mut(x) {
            *t = true;
        }
    }

    /// TBC: 0 clears the stop at the cursor, 3 clears them all.
    pub(super) fn clear_tabs(&mut self, mode: u16) {
        match mode {
            0 => {
                let x = self.scr_ref().cur.x;
                if let Some(t) = self.tabs.get_mut(x) {
                    *t = false;
                }
            }
            3 => self.tabs.iter_mut().for_each(|t| *t = false),
            _ => {}
        }
    }
}
