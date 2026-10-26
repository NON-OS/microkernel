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

//! A window's frame, pressed and moved.

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState, PointerAction};
use crate::input::InputKind;
use nonos_toolkit::decorations::{accessory_rect, Rect};

pub(super) const W: u32 = 700;

pub(super) const H: u32 = 400;

pub(super) fn press(state: &mut DragState, x: i32, y: i32) -> PointerAction {
    drag::handle(state, (W, H), (100, 100), false, &event(InputKind::ButtonDown, x, y))
}

pub(super) fn moved(action: PointerAction) -> Option<(u32, u32)> {
    match action {
        PointerAction::MoveTo(x, y) => Some((x, y)),
        _ => None,
    }
}

pub(super) fn tabs() -> Option<Rect> {
    accessory_rect(W, H, false, 278)
}
