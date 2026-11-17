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

//! Feeds the pointer and names what is under it, so a control draws hover and
//! press and a move repaints only on crossing to another target.

use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::{Pointer, State as Look};

use crate::viewer::target::{target_at, Shown, Target};

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

    /// Feeds a move, a press or a release; true when the window must be drawn
    /// again. The input router says the pointer left with a move to (-1, -1),
    /// which lands left of and above the content. A press or a release
    /// repaints the target it holds or lets go.
    pub fn feed(&mut self, shown: &Shown, ev: &InputEvent) -> bool {
        let p = &mut self.pointer;
        match ev.kind {
            InputKind::PointerAbs if ev.x < 0 && ev.y < 0 => p.left(),
            InputKind::PointerAbs => p.moved(ev.x, ev.y),
            InputKind::ButtonDown => p.pressed(ev.x, ev.y),
            InputKind::ButtonUp => p.released(),
            _ => return false,
        }
        let held = ev.kind != InputKind::PointerAbs && self.over.is_some();
        let moved = self.rehover(shown);
        held || moved
    }

    /// Names the target under the pointer again; true when that changed.
    pub fn rehover(&mut self, shown: &Shown) -> bool {
        let next = self.pointer.position().and_then(|(x, y)| target_at(shown, x, y));
        core::mem::replace(&mut self.over, next) != next
    }

    /// The hover and press the target `t` draws. Only the target under the
    /// hover asks the pointer, so a paint tests at most one target again.
    pub fn look(&self, shown: &Shown, t: Target) -> Look {
        if self.over != Some(t) {
            return Look::default();
        }
        self.pointer.state(|x, y| target_at(shown, x, y) == Some(t))
    }
}
