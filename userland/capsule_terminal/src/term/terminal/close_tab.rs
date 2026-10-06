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

//! Closing a tab, or the whole window, and hanging up the programs in it.

use nonos_app_skeleton::EventOutcome;

use super::types::Terminal;

impl Terminal {
    pub(super) fn close_tab(&mut self) -> EventOutcome {
        crate::jobs::hang_up(self.cur_ref());
        if self.tabs.len() <= 1 {
            return EventOutcome::Close;
        }
        self.tabs.remove(self.active);
        if self.active >= self.tabs.len() {
            self.active = self.tabs.len() - 1;
        }
        EventOutcome::Repaint
    }

    /// The window is closing: every tab's programs end with it.
    pub(super) fn hang_up_all(&self) {
        for tab in &self.tabs {
            crate::jobs::hang_up(tab);
        }
    }
}
