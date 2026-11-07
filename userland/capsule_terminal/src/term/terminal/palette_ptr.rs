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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use super::types::Terminal;
use crate::paint::palette_ptr::{feed, leave, Step};
use crate::paint::palette_view::{items, matches, view};
use crate::palette::Index;

impl Terminal {
    /// While the palette is open it takes every pointer event in the window,
    /// so nothing behind it is chosen, scrolled or clicked.
    pub(super) fn palette_pointer(&mut self, event: InputEvent) -> Option<EventOutcome> {
        if !self.palette.open || !event.is_pointer() {
            return None;
        }
        let body = self.layout.map(|l| l.body)?;
        let ix = Index::build(&self.tabs, self.active, self.prefs.project_slice());
        let rows = items(&matches(ix.slice(), self.palette.needle()));
        let step = feed(&mut self.palette, &view(body, &rows), &event);
        Some(match step {
            Step::Pick => self.palette_pick(),
            Step::Close => {
                self.palette.hide();
                EventOutcome::Repaint
            }
            Step::Repaint => EventOutcome::Repaint,
            Step::Idle => EventOutcome::Idle,
        })
    }

    /// The pointer moved onto the titlebar strip: true when that let go of a
    /// row held on the palette.
    pub(super) fn palette_leave(&mut self) -> bool {
        let Some(body) = self.layout.filter(|_| self.palette.open).map(|l| l.body) else {
            return false;
        };
        let ix = Index::build(&self.tabs, self.active, self.prefs.project_slice());
        let rows = items(&matches(ix.slice(), self.palette.needle()));
        leave(&mut self.palette, &view(body, &rows))
    }
}
