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
//! A press on the frame starts a resize on an edge or a move on the title
//! bar; the release commits the resize, once, so the surface is reallocated
//! a single time instead of on every step of the drag (which blinked).

use nonos_toolkit::decorations::{edges_at, hit_test_at, DecorationHit, Placed};

use super::state::{DragState, PointerAction};
use crate::input::InputEvent;

pub(in crate::runner) fn press(
    state: &mut DragState,
    (width, height): (u32, u32),
    (win_x, win_y): (u32, u32),
    maximized: bool,
    event: &InputEvent,
    quarters: u32,
) -> PointerAction {
    if event.x < 0 || event.y < 0 {
        return PointerAction::None;
    }
    let (x, y) = (event.x as u32, event.y as u32);
    (state.press_x, state.press_y) = (event.x, event.y);
    let edges = edges_at(width, height, maximized, x, y, quarters);
    if edges.any() {
        let base = Placed { x: win_x, y: win_y, w: width, h: height };
        state.resizing = Some((edges, base, base));
        state.active = false;
        return PointerAction::Held;
    } else if hit_test_at(width, height, maximized, x, y, quarters) == DecorationHit::Titlebar {
        state.active = true;
        state.resizing = None;
        (state.base_x, state.base_y) = (win_x, win_y);
    }
    PointerAction::None
}

/// The resize a drag ended on: held, and nothing applied, when it moved
/// nothing.
pub(in crate::runner) fn release(state: &mut DragState) -> PointerAction {
    state.active = false;
    let Some((edges, base, now)) = state.resizing.take() else {
        return PointerAction::None;
    };
    if now == base {
        PointerAction::Held
    } else if edges.left || edges.top {
        PointerAction::Place(now)
    } else {
        PointerAction::ResizeTo(now.w, now.h)
    }
}
