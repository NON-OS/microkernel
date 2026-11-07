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
//! The pointer moving over the frame: a held edge tracks the rect it would
//! leave (nothing is reallocated until the release), a held title bar moves
//! the window, and otherwise the control under the pointer lights.

use nonos_toolkit::decorations::{edges_at, hit_test_at, reshaped, DecorationHit, Edges};

use super::super::min_size::{MIN_H, MIN_W};
use super::state::{DragState, PointerAction};
use super::Grip;
use crate::input::InputEvent;

pub(in crate::runner) fn motion(
    state: &mut DragState,
    (width, height): (u32, u32),
    maximized: bool,
    event: &InputEvent,
    quarters: u32,
) -> PointerAction {
    let top = super::super::chrome::menubar_h();
    // The edges move by what the pointer moved since the press, not to the
    // pointer: a press inside the band must not jump the edge to it.
    let travel = (event.x as i64 - state.press_x as i64, event.y as i64 - state.press_y as i64);
    if let Some((edges, base, _)) = state.resizing {
        let now = reshaped(base, edges, travel, (MIN_W, MIN_H), top);
        state.resizing = Some((edges, base, now));
        return PointerAction::Held;
    }
    if state.active && maximized {
        return leave_maximized(state, width, event, travel);
    }
    if state.active {
        // A press that moved the window is a drag, not half a double click.
        if super::double::moved_off(travel) {
            state.title_press = None;
        }
        let nx = state.base_x as i64 + travel.0;
        let ny = state.base_y as i64 + travel.1;
        return PointerAction::MoveTo(nx.max(0) as u32, ny.max(top as i64) as u32);
    }
    let (hit, edges) = if event.x >= 0 && event.y >= 0 {
        let (x, y) = (event.x as u32, event.y as u32);
        (
            hit_test_at(width, height, maximized, x, y, quarters),
            edges_at(width, height, maximized, x, y, quarters),
        )
    } else {
        (DecorationHit::None, Edges::default())
    };
    state.cursor.0 = edges.cursor();
    if hit == state.hover {
        return PointerAction::None;
    }
    let was = core::mem::replace(&mut state.hover, hit);
    if is_control(hit) || is_control(was) {
        PointerAction::HoverChanged
    } else {
        PointerAction::None
    }
}

/// A held title bar of a maximized window: nothing until the pointer has
/// gone past the threshold, so a press that shakes may still be half a
/// double click, then the window leaves full screen under the pointer.
fn leave_maximized(
    state: &mut DragState,
    width: u32,
    event: &InputEvent,
    travel: (i64, i64),
) -> PointerAction {
    if !super::unmaximize::past(travel) {
        return PointerAction::Held;
    }
    state.title_press = None;
    let on_screen = |base: u32, at: i32| {
        let v = (i64::from(base) + i64::from(at)).max(0);
        u32::try_from(v).unwrap_or(u32::MAX)
    };
    PointerAction::Unmaximize(Grip {
        pointer: (on_screen(state.base_x, event.x), on_screen(state.base_y, event.y)),
        local: (event.x, event.y),
        press: (state.press_x, state.press_y),
        width,
    })
}

fn is_control(hit: DecorationHit) -> bool {
    !matches!(hit, DecorationHit::None | DecorationHit::Titlebar)
}
