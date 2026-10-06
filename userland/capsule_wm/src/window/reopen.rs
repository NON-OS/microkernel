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

//! A client opening a window this table still holds: its close never
//! arrived (or it opens again without one). The client has a new surface of
//! the size it asks for now, and puts it where the answer says.
//!
//! The answer was the old rect, size included, and the window kept the state
//! it had: a window left maximised came back at the whole work area over a
//! surface the size of the app's opening one, which the app then painted
//! past the end of row by row, and a window left minimised came back drawn
//! but skipped by the hit test, so presses on it went through it. It keeps
//! its place now, and takes the asked size and kind, on screen.

use super::{Kind, Visibility, Window};
use crate::geometry::{clamp_to_display, Rect};

pub fn reopen(window: &mut Window, kind: Kind, requested: Rect, display_w: u32, display_h: u32) {
    let at = Rect {
        x: window.rect.x,
        y: window.rect.y,
        width: requested.width,
        height: requested.height,
    };
    window.rect = clamp_to_display(at, display_w, display_h);
    window.kind = kind;
    window.visibility = Visibility::Visible;
    // The size asked for now is the client's new surface, not the full
    // screen it may have left, so the dock is no longer hidden for it.
    window.full_screen = false;
}
