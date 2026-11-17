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

//! The pointer over the tab strip: a move or a release repaints only when it
//! changes what the strip shows, and a menu open over it takes the pointer.

use alloc::vec::Vec;
use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};
use nonos_toolkit::widgets::{Pointer, State};

use super::app::Editor;
use super::shell::pane_x;
use super::strip::Spot;
use super::tabbar::{band, with_strip};

impl Editor {
    /// The pointer the strip draws with: none while a menu or panel is open.
    pub(super) fn strip_ptr(&self) -> Pointer {
        let menu = self.menu.is_some() || self.mb_open.is_some() || self.rb_open.is_some();
        if menu || self.panel.is_some() {
            Pointer::new()
        } else {
            self.st_ptr
        }
    }

    /// The control of the strip under `(x, y)`, as last painted.
    pub(super) fn strip_spot(&self, x: i32, y: i32) -> Option<Spot> {
        let at = band(pane_x(self.sidebar_open), self.last_w);
        with_strip(&self.docs, self.active, at, |s| s.spot(x, y))
    }

    fn strip_lit(&self) -> (Option<usize>, Vec<State>) {
        let (at, ptr) = (band(pane_x(self.sidebar_open), self.last_w), self.strip_ptr());
        with_strip(&self.docs, self.active, at, |s| s.lit(&ptr))
    }

    /// Feeds a pointer event to the strip: a repaint when a move or a release
    /// changes what it lights, else None so the event goes on as before.
    pub(super) fn strip_pointer(&mut self, event: &InputEvent) -> Option<EventOutcome> {
        let before = self.strip_lit();
        match event.kind {
            InputKind::ButtonDown => self.st_ptr.pressed(event.x, event.y),
            InputKind::PointerAbs => self.st_ptr.moved(event.x, event.y),
            InputKind::ButtonUp => self.st_ptr.released(),
            _ => return None,
        }
        let changed = event.kind != InputKind::ButtonDown && self.strip_lit() != before;
        changed.then_some(EventOutcome::Repaint)
    }
}
