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

//! Full screen: the green button's window, which fills the display below
//! the menubar down to its bottom edge, the band the dock is drawn in. The
//! desktop shell hides the dock while such a window shows, and is told so
//! through the lifecycle notifications (server/full_screen_notify.rs).
//!
//! The client says which it means. Maximise and restore are one request that
//! carries the rect to take, so the rect alone cannot tell a window made full
//! screen from one restored to a large saved size; the request's flag word
//! (`MAXIMIZE_FLAG_FULL_SCREEN`) does. A client that sends no flag, as every
//! client did before, gets the old behaviour: the window takes the rect and
//! the dock stays.

use super::{Visibility, Window, WindowTable};
use crate::geometry::{clamp_to_display, Rect};

/// The flag in window_maximize's second word that makes a window full screen.
pub const MAXIMIZE_FLAG_FULL_SCREEN: u32 = 1;

/// Whether `window` covers the dock's band now: made full screen and showing.
/// A minimised one does not, nor does one gone from the table.
pub fn covers_dock(window: &Window) -> bool {
    window.in_use && window.full_screen && window.visibility == Visibility::Visible
}

/// Whether `pid`'s window `window_id` covers the dock's band.
pub fn covers_dock_at(table: &WindowTable, pid: u32, window_id: u32) -> bool {
    table.find(pid, window_id).is_some_and(covers_dock)
}

/// window_maximize: the window takes `requested`, kept on the display (a full
/// screen rect, the whole width from the foot of the menubar to the bottom
/// edge, is on the display and so kept as asked), and is full screen when
/// `flags` says so, or no longer full screen when it does not: the green
/// button pressed again restores the saved rect with no flag.
pub fn maximize(window: &mut Window, requested: Rect, flags: u32, display_w: u32, display_h: u32) {
    window.rect = clamp_to_display(requested, display_w, display_h);
    window.full_screen = flags & MAXIMIZE_FLAG_FULL_SCREEN != 0;
}
