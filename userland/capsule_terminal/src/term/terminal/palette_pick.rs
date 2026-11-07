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
use crate::event::on_enter;
use crate::paint::palette_view::{chosen, items, matches, view};
use crate::palette::{Action, Index};
use crate::term::dimensions::COLS;

impl Terminal {
    /// Resolves the selection into an owned copy before acting, because the
    /// index borrows the tabs and the projects that the action then changes.
    pub(super) fn palette_pick(&mut self) -> EventOutcome {
        let mut buf = [0u8; COLS];
        let mut len = 0usize;
        let picked = {
            let ix = Index::build(&self.tabs, self.active, self.prefs.project_slice());
            let m = matches(ix.slice(), self.palette.needle());
            let rows = items(&m);
            let n = self.layout.map_or(m.len(), |l| view(l.body, &rows).rows);
            chosen(n, self.palette.sel).map(|i| {
                let e = m[i];
                len = e.label.len().min(COLS);
                buf[..len].copy_from_slice(&e.label.as_bytes()[..len]);
                e.action
            })
        };
        self.palette.hide();
        match picked {
            None => EventOutcome::Repaint,
            Some(Action::Run) => {
                self.cur().line.replace(&buf[..len]);
                on_enter(self.cur())
            }
            Some(Action::Session(i)) => self.palette_session(i as usize),
            Some(Action::Project(i)) => self.enter_project(i as usize),
            Some(Action::NewSession) => {
                self.open_tab();
                EventOutcome::Repaint
            }
            Some(Action::ToggleMonitor) => self.monitor_flip(),
            Some(Action::ChangeTheme) => self.theme_next(),
        }
    }
}
