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

//! Feeds the pointer and keeps the target under it, so a control draws hover
//! and press and a move repaints only on crossing to another target.

use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::{Pointer, State as Look};

use super::target::Target;

/// The pointer over the window, and the target under it.
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

    /// Feeds a move, a press or a release; true for a press or a release over
    /// a target, which must be drawn again. The input router says the pointer
    /// left with a move to (-1, -1), left of and above the content. Name the
    /// target under the pointer with `settle` after.
    pub fn feed(&mut self, ev: &InputEvent) -> bool {
        let p = &mut self.pointer;
        match ev.kind {
            InputKind::PointerAbs if ev.x < 0 && ev.y < 0 => p.left(),
            InputKind::PointerAbs => p.moved(ev.x, ev.y),
            InputKind::ButtonDown => p.pressed(ev.x, ev.y),
            InputKind::ButtonUp => p.released(),
            _ => return false,
        }
        ev.kind != InputKind::PointerAbs && self.over.is_some()
    }

    /// Takes `next` as the target under the pointer; true when that changed.
    pub fn settle(&mut self, next: Option<Target>) -> bool {
        core::mem::replace(&mut self.over, next) != next
    }

    /// The hover and press the target `t` draws, given what `at` names at a
    /// point. Only the target under the hover asks, so a paint names at most
    /// one target again.
    pub fn look(&self, t: Target, at: impl Fn(i32, i32) -> Option<Target>) -> Look {
        if self.over != Some(t) {
            return Look::default();
        }
        self.pointer.state(|x, y| at(x, y) == Some(t))
    }
}
