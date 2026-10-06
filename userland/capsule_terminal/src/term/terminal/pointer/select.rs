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

//! Routing pointer events over the body: to the program, the wheel, or
//! choosing text.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use crate::term::terminal::Terminal;

pub(super) const BTN_LEFT: u32 = 1;
pub(super) const BTN_RIGHT: u32 = 2;

impl Terminal {
    pub(crate) fn body_pointer(&mut self, event: InputEvent) -> Option<EventOutcome> {
        let g = self.cells?;
        let kinds =
            [InputKind::ButtonDown, InputKind::ButtonUp, InputKind::PointerAbs, InputKind::Wheel];
        if !kinds.contains(&event.kind) {
            return None;
        }
        let inside = g.inside(&self.cur_ref().scrollback.vt, event.x, event.y);
        if !inside && !self.ptr.selecting {
            return None;
        }
        if self.program_mouse(&event) {
            return Some(EventOutcome::Idle);
        }
        let pos = g.pos_at(&self.cur_ref().scrollback.vt, event.x, event.y);
        match event.kind {
            InputKind::Wheel => return Some(self.wheel(event)),
            InputKind::ButtonDown if event.code == BTN_LEFT => self.press(&event, pos),
            InputKind::PointerAbs if self.ptr.selecting => self.drag(pos),
            InputKind::ButtonUp if event.code == BTN_LEFT => self.ptr.selecting = false,
            _ => return Some(EventOutcome::Idle),
        }
        Some(EventOutcome::Repaint)
    }
}
