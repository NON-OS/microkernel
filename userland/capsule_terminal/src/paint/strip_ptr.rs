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

//! The pointer over the titlebar strip: which control it is on, what that
//! control draws, and which moves are worth a repaint.

use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Pointer, Rect, State};

use super::strip_spot::{spot, Spot, Step};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StripPointer {
    pub ptr: Pointer,
    /// The strip's size as the frame last granted it.
    pub size: (u32, u32),
}

impl StripPointer {
    /// The control under the pointer, if the pointer is over the strip.
    pub fn spot(&self, names: &[&str], active: usize) -> Option<Spot> {
        let (x, y) = self.ptr.position()?;
        spot(names, active, self.size, x, y)
    }

    /// The hover and press the control drawn in `r` shows.
    pub fn look(&self, r: Rect) -> State {
        self.ptr.state(|x, y| contains(r, x, y))
    }

    /// A press acts on the control under it; a move or a release repaints
    /// only when it changes what is lit.
    pub fn feed(&mut self, names: &[&str], active: usize, event: &InputEvent) -> Step {
        let before = self.spot(names, active);
        match event.kind {
            InputKind::PointerAbs => self.ptr.moved(event.x, event.y),
            InputKind::ButtonUp => self.ptr.released(),
            InputKind::ButtonDown => {
                self.ptr.pressed(event.x, event.y);
                if let Some(s) = self.spot(names, active) {
                    return Step::Act(s);
                }
            }
            _ => return Step::Idle,
        }
        let freed = event.kind == InputKind::ButtonUp && before.is_some();
        let lit = self.spot(names, active) != before || freed;
        if lit {
            Step::Repaint
        } else {
            Step::Idle
        }
    }

    /// The pointer left the strip: true when that unlit a control.
    pub fn leave(&mut self, names: &[&str], active: usize) -> bool {
        let lit = self.spot(names, active).is_some();
        self.ptr.left();
        lit
    }
}
