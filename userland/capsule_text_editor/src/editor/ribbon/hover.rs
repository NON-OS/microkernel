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

//! The pointer over the ribbon's square controls: a move or a release
//! repaints only when it changes what a square shows. A menu open over the
//! ribbon takes the pointer, so no square lights beneath it.

use alloc::vec::Vec;
use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};
use nonos_toolkit::widgets::{Pointer, State};

use super::snapshot::ribbon_state;
use super::squares::square;
use crate::editor::app::Editor;

impl Editor {
    /// The pointer the ribbon's squares draw with: none while a menu is open.
    pub(in crate::editor) fn ribbon_ptr(&self) -> Pointer {
        let covered = self.menu.is_some() || self.mb_open.is_some() || self.panel.is_some();
        if covered {
            Pointer::new()
        } else {
            self.rb_ptr
        }
    }

    fn ribbon_looks(&self) -> Vec<State> {
        let (st, ptr) = (ribbon_state(self.doc_ref()), self.ribbon_ptr());
        let squares = self.rb_layout.iter().filter_map(|c| square(c, &st));
        squares.map(|s| s.state(&ptr)).collect()
    }

    /// Feeds a pointer event to the squares: a repaint when a move or a
    /// release changes one, else None so the event goes on as before. A press
    /// goes on to the ribbon's own press handling, which repaints.
    pub(in crate::editor) fn ribbon_pointer(&mut self, event: &InputEvent) -> Option<EventOutcome> {
        let before = self.ribbon_looks();
        match event.kind {
            InputKind::ButtonDown => self.rb_ptr.pressed(event.x, event.y),
            InputKind::PointerAbs => self.rb_ptr.moved(event.x, event.y),
            InputKind::ButtonUp => self.rb_ptr.released(),
            _ => return None,
        }
        let changed = event.kind != InputKind::ButtonDown && self.ribbon_looks() != before;
        changed.then_some(EventOutcome::Repaint)
    }
}
