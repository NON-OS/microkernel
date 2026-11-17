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

//! Feeds the pointer and keeps the control under it, so a control draws hover
//! and press and a move repaints only on crossing to another control.

use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::{Pointer, State as Look};

use super::shown::{target_at, Shown};
use super::target::Target;

/// The pointer over the window, and the control under it.
#[derive(Clone, Copy, Debug)]
pub struct Hover {
    pub pointer: Pointer,
    pub over: Option<Target>,
}

impl Hover {
    /// A pointer not yet seen, over nothing.
    pub const fn new() -> Self {
        Hover { pointer: Pointer::new(), over: None }
    }

    /// Feeds a move or a release, neither of which clicks; false for any
    /// other event. The input router says the pointer left with a move to
    /// (-1, -1), left of and above the content. Name the control under the
    /// pointer with `settle` after.
    pub fn feed(&mut self, ev: &InputEvent) -> bool {
        let p = &mut self.pointer;
        match ev.kind {
            InputKind::PointerAbs if ev.x < 0 && ev.y < 0 => p.left(),
            InputKind::PointerAbs => p.moved(ev.x, ev.y),
            InputKind::ButtonUp => p.released(),
            _ => return false,
        }
        true
    }

    /// Names the control under the pointer in `s` again; true when it changed.
    pub fn settle(&mut self, s: &Shown) -> bool {
        let next = self.pointer.position().and_then(|(x, y)| target_at(s, x, y));
        core::mem::replace(&mut self.over, next) != next
    }
}

/// The hover and the window it is over, which the painters ask for each
/// control's look.
pub struct Lit<'a>(pub &'a Hover, pub &'a Shown);

impl Lit<'_> {
    /// The hover and press the control `t` draws. Only the control under the
    /// hover asks the pointer, so a paint names at most one control again.
    pub fn of(&self, t: Target) -> Look {
        if self.0.over != Some(t) {
            return Look::default();
        }
        self.0.pointer.state(|x, y| target_at(self.1, x, y) == Some(t))
    }
}
