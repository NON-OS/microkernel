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

//! A press and drag on the frame: the title bar moves the window, an edge
//! or corner resizes it (toolkit decorations/edges.rs), and the pointer's
//! hover over the controls lights them.

use super::state::{DragState, PointerAction};
use crate::input::{InputEvent, InputKind};

pub(in crate::runner) fn handle(
    state: &mut DragState,
    (width, height): (u32, u32),
    (win_x, win_y): (u32, u32),
    maximized: bool,
    event: &InputEvent,
) -> PointerAction {
    let q = super::super::chrome::quarters();
    handle_at(state, (width, height), (win_x, win_y), maximized, event, q)
}

/// The same, with the frame at `quarters` of display scale.
pub(in crate::runner) fn handle_at(
    state: &mut DragState,
    size: (u32, u32),
    at: (u32, u32),
    maximized: bool,
    event: &InputEvent,
    quarters: u32,
) -> PointerAction {
    match event.kind {
        InputKind::ButtonDown => super::press::press(state, size, at, maximized, event, quarters),
        InputKind::ButtonUp => super::press::release(state),
        InputKind::PointerAbs => super::motion::motion(state, size, maximized, event, quarters),
        _ => PointerAction::None,
    }
}
