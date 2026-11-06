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

//! The pointer's part of the window's input. Moves and releases feed the
//! hover, a press feeds it before the click acts, and after any event the
//! control under the pointer is named again, since a click, a key or the
//! wheel can change what lies there.

use nonos_app_skeleton::EventOutcome::{self, Idle, Repaint};
use nonos_app_skeleton::{InputEvent, InputKind};

use super::PlayerApp;
use crate::ui;

impl PlayerApp {
    /// Names the control under the pointer again; true when that changed.
    fn rehover(&mut self) -> bool {
        let rows = self.rows();
        let (queue, n) = (self.queue.items(), self.library.tracks.len());
        let lists = ui::Lists { rows: &rows, queue, n, downloads: self.downloads.rows() };
        let on_show = ui::shown(&self.ui, self.dims, &lists);
        self.ui.hover.settle(&on_show)
    }

    /// Moves and releases, neither of which clicks; None for any other event.
    /// A release repaints what may have been held.
    pub(super) fn on_pointer(&mut self, ev: &InputEvent) -> Option<EventOutcome> {
        if !self.ui.hover.feed(ev) {
            return None;
        }
        let held = ev.kind == InputKind::ButtonUp && self.ui.hover.over.is_some();
        let moved = self.rehover();
        Some(if held || moved { Repaint } else { Idle })
    }

    /// A press: the pointer holds it from here, and the click acts. A press
    /// on a control repaints it held even where the click asks nothing.
    pub(super) fn on_press(&mut self, x: i32, y: i32) -> EventOutcome {
        self.ui.hover.pointer.pressed(x, y);
        let out = self.on_click(x, y);
        if out == Idle && self.ui.hover.over.is_some() {
            return Repaint;
        }
        out
    }

    /// `out`, lifted from Idle to a repaint when the event moved what lies
    /// under the pointer. No other outcome is changed.
    pub(super) fn settled(&mut self, out: EventOutcome) -> EventOutcome {
        let moved = self.rehover();
        if out == Idle && moved {
            return Repaint;
        }
        out
    }
}
