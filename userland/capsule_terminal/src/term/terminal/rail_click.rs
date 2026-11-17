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

use alloc::vec::Vec;
use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::types::Terminal;
use crate::paint::rail_ptr::{feed, leave, Step};
use crate::paint::rail_spot::Spot;

impl Terminal {
    /// A pointer event on the rail lights, presses or acts on the control
    /// under it. The column comes from the cached solve, so the row a click
    /// resolves to is the row the last frame drew there. A selection being
    /// dragged keeps the pointer, as it did before the rail took moves.
    pub(super) fn rail_pointer(&mut self, event: InputEvent) -> Option<EventOutcome> {
        let kinds = [InputKind::PointerAbs, InputKind::ButtonDown, InputKind::ButtonUp];
        let c = self.rail_column()?;
        let on = c.local(event.x, event.y).is_some();
        if !kinds.contains(&event.kind) || self.ptr.selecting || !on {
            return None;
        }
        Some(match feed(&mut self.rail_ptr, &c, &event) {
            Step::Act(Spot::NewSession) => {
                self.open_tab();
                EventOutcome::Repaint
            }
            Step::Act(Spot::AddProject) => self.pin_cwd(),
            Step::Act(Spot::Session(i)) => {
                self.active = (i as usize).min(self.tabs.len() - 1);
                EventOutcome::Repaint
            }
            Step::Act(Spot::Project(i)) => self.enter_project(i as usize),
            Step::Repaint => EventOutcome::Repaint,
            Step::Idle => EventOutcome::Idle,
        })
    }

    /// The pointer is somewhere off the rail: true when that unlit a control.
    pub(super) fn rail_leave(&mut self, event: Option<&InputEvent>) -> bool {
        let Some(c) = self.rail_column() else { return false };
        if event.is_some_and(|e| c.local(e.x, e.y).is_some()) {
            return false;
        }
        leave(&mut self.rail_ptr, &c)
    }

    fn pin_cwd(&mut self) -> EventOutcome {
        let path = Vec::from(self.cur_ref().cwd.as_bytes());
        if !self.prefs.push_project(&path) {
            return EventOutcome::Idle;
        }
        self.prefs_dirty = true;
        EventOutcome::Repaint
    }

    pub(super) fn enter_project(&mut self, i: usize) -> EventOutcome {
        let Some(path) = self.prefs.project_slice().get(i).map(|p| Vec::from(p.as_bytes())) else {
            return EventOutcome::Idle;
        };
        self.cur().cwd.set(path);
        EventOutcome::Repaint
    }
}
