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

//! What a change of the lit window control changes in the window: the
//! control put out and the one lit (toolkit decorations/controls.rs washes
//! and inks each inside its own target), nothing else. The title bar and
//! the content draw the same whether the pointer is over them or not.
//!
//! A frame whose hover alone changed was drawn and presented whole, so on a
//! maximized window each control the pointer crossed composed the whole
//! 1920x1022 window, folded with the menu bar into the whole screen. The
//! window is still drawn whole into the buffer it takes, so that buffer
//! holds the whole picture, and only this part of it is presented.

use nonos_toolkit::decorations::{control_rect_at, DecorationHit, FrameState, Rect, CONTROLS};

/// A frame as painted: width, height, the frame's state, maximized.
pub type Painted = (u32, u32, FrameState, bool);

/// When `now` differs from `was` in the hover alone, the part to present:
/// the box round the control put out and the one lit, or None when neither
/// is a control. None outside: the size, the focus or maximized changed
/// (or nothing did), and the frame is shown whole or by the app's damage.
pub fn hover_part(was: Painted, now: Painted, quarters: u32) -> Option<Option<Rect>> {
    let (w, h, s, max) = now;
    let only_hover = (was.0, was.1, was.2.focused, was.3) == (w, h, s.focused, max);
    if !only_hover || was.2.hover == s.hover {
        return None;
    }
    let lit = |hit: DecorationHit| {
        let i = CONTROLS.iter().position(|c| *c == hit)?;
        Some(control_rect_at(i as u32, w, h, max, quarters))
    };
    Some(match (lit(was.2.hover), lit(s.hover)) {
        (Some(a), Some(b)) => Some(union(a, b)),
        (a, b) => a.or(b),
    })
}

/// The box round two rectangles.
pub fn union(a: Rect, b: Rect) -> Rect {
    let (x, y) = (a.x.min(b.x), a.y.min(b.y));
    let right = a.x.saturating_add(a.w).max(b.x.saturating_add(b.w));
    let bottom = a.y.saturating_add(a.h).max(b.y.saturating_add(b.h));
    Rect { x, y, w: right - x, h: bottom - y }
}
