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

//! The pointer over the command palette: hovering a row selects it, a press
//! held on a row picks it when it comes up there, and a press off the field
//! and the menu closes the palette.

use nonos_app_skeleton::{InputEvent, InputKind};
use nonos_toolkit::widgets::rect::contains;

use super::palette_view::{lit, View};
use crate::palette::Palette;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Step {
    Idle,
    Repaint,
    /// Act on the selection, which the release has just set.
    Pick,
    Close,
}

/// Feeds one pointer event to the palette drawn as `v`. A move or a press
/// repaints only when it changes the washed or the held row.
pub fn feed(pal: &mut Palette, v: &View, e: &InputEvent) -> Step {
    let before = lit(v, &pal.ptr, pal.sel);
    let row = v.menu.hit(v.at, e.x, e.y);
    match e.kind {
        InputKind::PointerAbs => pal.ptr.moved(e.x, e.y),
        InputKind::ButtonDown if !contains(v.field, e.x, e.y) && !contains(v.at, e.x, e.y) => {
            return Step::Close;
        }
        InputKind::ButtonDown => pal.ptr.pressed(e.x, e.y),
        InputKind::ButtonUp => {
            let held = before.1;
            pal.ptr.released();
            if let Some(i) = held {
                pal.sel = i;
                return Step::Pick;
            }
        }
        _ => return Step::Idle,
    }
    if let Some(i) = row.filter(|_| e.kind != InputKind::ButtonUp) {
        pal.sel = i;
    }
    if lit(v, &pal.ptr, pal.sel) != before {
        Step::Repaint
    } else {
        Step::Idle
    }
}

/// The pointer left the palette's window: true when that let go of a row.
pub fn leave(pal: &mut Palette, v: &View) -> bool {
    let held = lit(v, &pal.ptr, pal.sel).1.is_some();
    pal.ptr.left();
    held
}
