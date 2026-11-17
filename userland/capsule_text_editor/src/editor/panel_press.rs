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

//! An open panel takes the whole next press: a Special Character row inserts
//! its text at the caret, anything else just dismisses the panel. Its rows
//! are fed the pointer through the controls the painter laid them out in.

use nonos_app_skeleton::{feed, EventOutcome, InputEvent, InputKind};
use nonos_toolkit::widgets::Step;

use super::app::Editor;
use super::panel::Panel;
use super::specials::SPECIALS;

impl Editor {
    pub(super) fn open_panel(&mut self, panel: Panel) {
        self.panel = Some(panel);
    }

    /// Feeds a pointer event to the open panel's rows. With none open the
    /// pointer has left them.
    pub(super) fn panel_step(&mut self, event: &InputEvent) -> Step<usize> {
        if self.panel.is_none() {
            return self.panel_ctl.left();
        }
        feed(&mut self.panel_ctl, event)
    }

    /// What an open panel makes of `step`: a press anywhere closes it, a
    /// picked row first inserting its text, and a row lighting repaints.
    pub(super) fn panel_event(
        &mut self,
        event: &InputEvent,
        step: Step<usize>,
    ) -> Option<EventOutcome> {
        self.panel?;
        if event.kind != InputKind::ButtonDown {
            return (step == Step::Repaint).then_some(EventOutcome::Repaint);
        }
        self.panel = None;
        if let Some((_, text)) = match step {
            Step::Act(i) => SPECIALS.get(i),
            _ => None,
        } {
            let doc = self.doc();
            if !doc.insert(text.as_bytes()) {
                doc.status = crate::editor::unsupported::NO_ROOM;
            }
        }
        Some(EventOutcome::Repaint)
    }
}
