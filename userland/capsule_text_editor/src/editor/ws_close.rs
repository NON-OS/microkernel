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

//! Closing tabs, and the window, without losing unsaved text.

use super::app::Editor;
use super::state::State;

impl Editor {
    /// Close a tab the user asked to close. Unsaved text costs a second
    /// request on the same tab, from any close control, because closing
    /// cannot be undone. Returns whether the tab went.
    pub(super) fn request_close(&mut self, idx: usize) -> bool {
        let Some(d) = self.docs.get_mut(idx) else { return false };
        if d.dirty && self.close_armed != Some(idx) {
            self.close_armed = Some(idx);
            d.status = b"unsaved changes: close again to discard them, Ctrl+S to save";
            self.active = idx;
            return false;
        }
        self.close_armed = None;
        self.close_tab(idx);
        true
    }

    fn close_tab(&mut self, idx: usize) {
        if idx >= self.docs.len() {
            return;
        }
        self.docs.remove(idx);
        if self.docs.is_empty() {
            self.docs.push(State::new());
            self.active = 0;
            return;
        }
        if self.active > idx || self.active >= self.docs.len() {
            self.active = self.active.saturating_sub(1).min(self.docs.len() - 1);
        }
    }

    /// The window close button: unsaved text anywhere costs a second press,
    /// with the first unsaved tab brought to the front to say so.
    pub(super) fn confirm_quit(&mut self) -> bool {
        let unsaved = self.docs.iter().position(|d| d.dirty);
        match unsaved {
            Some(i) if !self.quit_armed => {
                self.quit_armed = true;
                self.active = i;
                self.docs[i].status = b"unsaved changes: close the window again to discard them";
                false
            }
            _ => true,
        }
    }
}
