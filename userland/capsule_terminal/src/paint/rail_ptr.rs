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

//! The pointer over the rail: a press acts on the control under it, and a
//! move, a release or leaving repaints only when it changes what is lit.

use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::Pointer;

use super::rail_column::Column;
use super::rail_spot::Spot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Step {
    Idle,
    Repaint,
    Act(Spot),
}

/// Feeds one pointer event that landed on the rail drawn as `c`.
pub fn feed(ptr: &mut Pointer, c: &Column, e: &InputEvent) -> Step {
    let before = c.lit(ptr);
    match e.kind {
        InputKind::PointerAbs => ptr.moved(e.x, e.y),
        InputKind::ButtonUp => ptr.released(),
        InputKind::ButtonDown => {
            ptr.pressed(e.x, e.y);
            if let Some(s) = c.spot(e.x, e.y) {
                return Step::Act(s);
            }
        }
        _ => return Step::Idle,
    }
    let freed = e.kind == InputKind::ButtonUp && before.is_some();
    if c.lit(ptr) != before || freed {
        Step::Repaint
    } else {
        Step::Idle
    }
}

/// The pointer went somewhere off the rail: true when that unlit a control.
pub fn leave(ptr: &mut Pointer, c: &Column) -> bool {
    let lit = c.lit(ptr).is_some();
    ptr.left();
    lit
}
