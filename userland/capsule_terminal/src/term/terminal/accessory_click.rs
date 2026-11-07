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

use nonos_app_skeleton::EventOutcome;

use super::types::Terminal;
use crate::paint::strip_spot::Spot;
use crate::term::theme::choice;

impl Terminal {
    /// What a press on a strip control does. The search button opens the
    /// command palette, which is what searching means here: one field over
    /// commands, tabs and projects, where Ctrl-R already searches history.
    pub(super) fn strip_act(&mut self, at: Spot) -> EventOutcome {
        match at {
            Spot::Tool(0) | Spot::Plus => self.open_tab(),
            Spot::Tool(1) => self.palette.show(),
            Spot::Tool(_) => self.theme = choice::next(self.theme),
            Spot::Close(i) => {
                self.active = i;
                return self.close_tab();
            }
            Spot::Tab(i) => self.active = i,
        }
        EventOutcome::Repaint
    }
}
