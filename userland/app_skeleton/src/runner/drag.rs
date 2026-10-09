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

use nonos_toolkit::decorations::{hit_test_at, margin_at, scaled, titlebar_rect_at, DecorationHit};

use super::press_part::PressGrab;
use crate::input::{InputEvent, InputKind};

use super::min_size::{MIN_H, MIN_W};

const EDGE: u32 = 10; // grab band on the right / bottom borders
const STEP: u32 = 8; // only resize once the drag moves this far (throttle reallocs)

pub(super) struct DragState {
    pub active: bool,
    pub hover: DecorationHit,
    /// Which part of the window holds the press, so a drag keeps every event
    /// until its release (see press_part).
    pub press: PressGrab,
    press_x: i32,
    press_y: i32,
    base_x: u32,
    base_y: u32,
    resizing: bool,
    rz_right: bool,
    rz_bottom: bool,
    last_w: u32,
    last_h: u32,
}

impl DragState {
    pub(super) const fn new() -> Self {
        Self {
            active: false,
            hover: DecorationHit::None,
            press: PressGrab::new(),
            press_x: 0,
            press_y: 0,
            base_x: 0,
            base_y: 0,
            resizing: false,
            rz_right: false,
            rz_bottom: false,
            last_w: 0,
            last_h: 0,
        }
    }
}

pub(super) enum PointerAction {
    None,
    MoveTo(u32, u32),
    ResizeTo(u32, u32),
    HoverChanged,
}

pub(super) fn handle(
    state: &mut DragState,
    width: u32,
    height: u32,
    win_x: u32,
    win_y: u32,
    maximized: bool,
    event: &InputEvent,
) -> PointerAction {
    let q = super::chrome::quarters();
    handle_at(state, (width, height), (win_x, win_y), maximized, event, q)
}

/// The same, with the frame at `quarters` of display scale: the border band
/// that resizes and the title bar that moves are measured on the frame as
/// drawn at that scale.
pub(super) fn handle_at(
    state: &mut DragState,
    (width, height): (u32, u32),
    (win_x, win_y): (u32, u32),
    maximized: bool,
    event: &InputEvent,
    quarters: u32,
) -> PointerAction {
    let m = margin_at(maximized, quarters);
    let edge = scaled(EDGE, quarters);
    match event.kind {
        InputKind::ButtonDown => {
            if event.x >= 0 && event.y >= 0 {
                let (x, y) = (event.x as u32, event.y as u32);
                let on_right = x + edge + m >= width;
                let on_bottom = y + edge + m >= height;
                // Resize grabs the right/bottom borders (below the titlebar);
                // the titlebar itself still moves the window. Below means past
                // the titlebar's own bottom edge, in the window's coordinates
                // the press arrives in. This compared against the menubar's
                // height instead, a screen measure, so the right end of the
                // titlebar's lowest pixels started a resize, not a move.
                let bar = titlebar_rect_at(width, height, maximized, quarters);
                let below_bar = y >= bar.y.saturating_add(bar.h);
                if (on_right || on_bottom) && below_bar {
                    state.resizing = true;
                    state.rz_right = on_right;
                    state.rz_bottom = on_bottom;
                    state.press_x = event.x;
                    state.press_y = event.y;
                    state.last_w = width;
                    state.last_h = height;
                    state.active = false;
                    return PointerAction::None;
                }
                if hit_test_at(width, height, maximized, x, y, quarters) == DecorationHit::Titlebar
                {
                    state.active = true;
                    state.resizing = false;
                    state.press_x = event.x;
                    state.press_y = event.y;
                    state.base_x = win_x;
                    state.base_y = win_y;
                }
            }
            PointerAction::None
        }
        InputKind::ButtonUp => {
            state.active = false;
            // Commit the resize once, on release, so the surface is reallocated
            // a single time instead of on every drag step (which blinked).
            if state.resizing {
                state.resizing = false;
                if state.last_w >= MIN_W && state.last_h >= MIN_H {
                    return PointerAction::ResizeTo(state.last_w, state.last_h);
                }
            }
            PointerAction::None
        }
        InputKind::PointerAbs if state.resizing => {
            // Track the target size during the drag; do NOT reallocate yet.
            // The border moves by what the pointer moved since the press. It
            // was put at the pointer plus the frame's margin instead, so the
            // first step of every resize jumped the border by however far
            // inside the grab band the press had landed.
            let grow = |size: u32, now: i32, at: i32, min: u32| {
                (size as i64 + (now as i64 - at as i64)).clamp(min as i64, u32::MAX as i64) as u32
            };
            let nw =
                if state.rz_right { grow(width, event.x, state.press_x, MIN_W) } else { width };
            let nh =
                if state.rz_bottom { grow(height, event.y, state.press_y, MIN_H) } else { height };
            state.last_w = nw;
            state.last_h = nh;
            let _ = STEP;
            PointerAction::None
        }
        InputKind::PointerAbs if state.active => {
            let nx = state.base_x as i64 + (event.x - state.press_x) as i64;
            let ny = state.base_y as i64 + (event.y - state.press_y) as i64;
            let top = super::chrome::menubar_h() as i64;
            PointerAction::MoveTo(nx.max(0) as u32, ny.max(top) as u32)
        }
        InputKind::PointerAbs => {
            let hit = if event.x >= 0 && event.y >= 0 {
                hit_test_at(width, height, maximized, event.x as u32, event.y as u32, quarters)
            } else {
                DecorationHit::None
            };
            if hit == state.hover {
                return PointerAction::None;
            }
            let was = state.hover;
            state.hover = hit;
            if is_button(hit) || is_button(was) {
                PointerAction::HoverChanged
            } else {
                PointerAction::None
            }
        }
        _ => PointerAction::None,
    }
}

fn is_button(hit: DecorationHit) -> bool {
    matches!(
        hit,
        DecorationHit::CloseButton | DecorationHit::MinimizeButton | DecorationHit::MaximizeButton
    )
}
