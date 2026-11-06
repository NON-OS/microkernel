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
//! hover, and after any event the control under the pointer is named again,
//! since a click, a key or the wheel can change what lies there.

use nonos_app_skeleton::EventOutcome::{self, Idle, Repaint};
use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::State;

use crate::app::state::VideoApp;
use crate::ui::hover::Lit;
use crate::ui::target::Target;
use crate::ui::targets::Shown;

impl VideoApp {
    /// What the window shows that decides where its controls sit.
    pub(crate) fn shown(&self) -> Shown {
        let (b, route, dims) = (&self.browse, self.route(), self.dims);
        Shown { route, dims, grid: b.grid, scroll: b.scroll, len: b.len() }
    }

    /// The hover and press the control `t` draws.
    pub(crate) fn look(&self, t: Target) -> State {
        Lit(&self.hover, &self.shown()).of(t)
    }

    /// Names the control under the pointer again; true when that changed.
    fn rehover(&mut self) -> bool {
        let shown = self.shown();
        self.hover.settle(&shown)
    }

    /// Moves and releases, neither of which clicks; None for any other event.
    /// A release repaints what may have been held.
    pub(super) fn on_pointer(&mut self, ev: &InputEvent) -> Option<EventOutcome> {
        if !self.hover.feed(ev) {
            return None;
        }
        let held = ev.kind == InputKind::ButtonUp && self.hover.over.is_some();
        let moved = self.rehover();
        Some(if held || moved { Repaint } else { Idle })
    }

    /// `out`, lifted from Idle to a repaint when the event moved what lies
    /// under the pointer, or was a press on a control, which draws held. No
    /// other outcome is changed.
    pub(super) fn settled(&mut self, out: EventOutcome, press: bool) -> EventOutcome {
        let moved = self.rehover();
        let held = press && self.hover.over.is_some();
        if out == Idle && (moved || held) {
            return Repaint;
        }
        out
    }
}
