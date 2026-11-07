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

//! The banner's uptime row, the one part of a new tab's splash that changes
//! by itself. A tick that changed it and nothing else (rail_repaint.rs) has
//! the window drawn whole, so the buffer holds a complete picture, and only
//! this row presented, where the whole window was composed every second.

use crate::layout::Rect;

/// The band a fact row is drawn in, from its left edge `x` to the panel's
/// `right`, `h` high; None when it starts above or left of the buffer or has
/// no width.
pub fn row_rect(x: i32, y: i32, right: i32, h: u32) -> Option<Rect> {
    let w = u32::try_from(right.checked_sub(x)?).ok().filter(|&w| w > 0)?;
    Some(Rect { x: u32::try_from(x).ok()?, y: u32::try_from(y).ok()?, w, h })
}

/// The frame's damage for a tick that changed the uptime row alone: the row
/// where the splash last drew it, unless a change no frame has drawn whole is
/// still owed to the screen, which would then never reach it.
pub fn uptime_damage(uptime_only: bool, whole_owed: bool, row: Option<Rect>) -> Option<Rect> {
    row.filter(|_| uptime_only && !whole_owed)
}
