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
use nonos_toolkit::widgets::Menu;

use super::types::Terminal;
use crate::paint::palette_view::{chosen, items, matches, view};
use crate::palette::Index;
use crate::term::theme::choice;

impl Terminal {
    /// Moves the washed row the way the menu's arrow keys do, wrapping
    /// among the rows the menu shows.
    pub(super) fn palette_step(&mut self, delta: i32) -> EventOutcome {
        let ix = Index::build(&self.tabs, self.active, self.prefs.project_slice());
        let rows = items(&matches(ix.slice(), self.palette.needle()));
        let n = self.layout.map_or(rows.len(), |l| view(l.body, &rows).rows);
        let from = chosen(n, self.palette.sel);
        self.palette.sel = Menu::next(&rows[..n], from, delta).unwrap_or(0);
        EventOutcome::Repaint
    }

    pub(super) fn palette_session(&mut self, i: usize) -> EventOutcome {
        self.active = i.min(self.tabs.len().saturating_sub(1));
        EventOutcome::Repaint
    }

    /// The right rail is the monitor. Hiding it is a preference, so the record
    /// carries it and the next launch opens the way this one was left.
    pub(super) fn monitor_flip(&mut self) -> EventOutcome {
        self.prefs.rails ^= 1;
        self.prefs_dirty = true;
        EventOutcome::Repaint
    }

    pub(super) fn theme_next(&mut self) -> EventOutcome {
        self.theme = choice::next(self.theme);
        self.prefs_dirty = true;
        EventOutcome::Repaint
    }
}
