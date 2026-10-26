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

//! A window gone in the middle of a run of presses.

use super::order::Win;
use super::session::Session;

impl Session {
    /// The window leaves the strip. The run stays on the window it was on;
    /// when that is the one gone, the next press in the run's direction
    /// lands where a press past the gone one would have.
    pub(super) fn forget(&mut self, w: Win) {
        let Some(i) = self.windows.iter().position(|&x| x == w) else { return };
        self.windows.remove(i);
        let n = self.windows.len();
        let at = self.at.min(n);
        if i < at || (i == at && !self.back) {
            self.at = if at == 0 { n.saturating_sub(1) } else { at - 1 };
        } else {
            self.at = at;
        }
    }
}
