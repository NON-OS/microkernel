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
//! A pointer event on a popup in a window of its own (popup/window.rs): the
//! router gives it in the popup window's own pixels, and it is taken as the
//! same point on the toplevel's surface, where the popups' places are kept
//! (popup/stack.rs), so finding the popup under it, entering it and its
//! grab go as for a popup laid on its window. The NØNOS frame is not on it.

use nonos_app_skeleton::input::{InputEvent, InputKind};

use crate::linux::guest::Guest;

use super::super::present::scale_math::to_logical;
use super::axis::wheel;
use super::button::{button, press_at};
use super::send::motion;

/// The point (`x`, `y`) on popup window `id` as one on its toplevel's
/// surface, its window brought to `Scene.win`; None when `id` is no popup.
pub fn point(guest: &mut Guest, id: u32, (x, y): (i32, i32)) -> Option<(i32, i32)> {
    let (surface, rect) = super::super::popup::window::of_window(&guest.scene, id)?;
    super::super::window_front::select_for(guest, surface);
    let e = guest.scene.win.frame.ratio;
    Some((rect.0.saturating_add(to_logical(x, e)), rect.1.saturating_add(to_logical(y, e))))
}

/// `event` at (`x`, `y`) on the toplevel's surface.
pub fn deliver(guest: &mut Guest, event: &InputEvent, (x, y): (i32, i32)) {
    match event.kind {
        InputKind::PointerAbs => motion(guest, x, y),
        InputKind::ButtonDown => press_at(guest, event.code, (x, y)),
        InputKind::ButtonUp => button(guest, event.code, 0),
        InputKind::Wheel => wheel(guest, event.delta_x, event.delta_y, x, y),
        _ => {}
    }
}
