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
//! A drag of a maximized window's title bar: once the pointer has moved
//! past `UNMAXIMIZE_PX` from its press, the window goes back to the size it
//! had before it was maximized, put where the pointer holds the title bar
//! at the same share of its width, and the drag goes on moving it from
//! there. A drag of a maximized window used to do nothing: every step was
//! clamped back to the full-screen rect. Pure, so the host proofs hold it
//! (desktop_proofs drag_tests unmaximize).

/// How far the pointer must move, in pixels each way, before a drag takes
/// a maximized window out of it: a press that only shakes stays a press,
/// and may still be half a double click.
pub(in crate::runner) const UNMAXIMIZE_PX: u32 = 4;

/// Where the pointer held a maximized window's title bar when the drag
/// left it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::runner) struct Grip {
    /// The pointer on the screen.
    pub pointer: (u32, u32),
    /// The pointer in the frame the press was armed in: the router keeps
    /// giving the drag's events in it after the window moves.
    pub local: (i32, i32),
    /// Where in the maximized window the press was, and that window's width.
    pub press: (i32, i32),
    pub width: u32,
}

impl Grip {
    /// The origin of a window `saved_w` wide that keeps the pointer at the
    /// same share of the title bar's width, at the same height in it,
    /// never above `top` and never past the right edge of a screen
    /// `self.width` wide when the window fits on it.
    pub(in crate::runner) fn restored_at(&self, saved_w: u32, top: u32) -> (u32, u32) {
        let width = u64::from(self.width);
        let along = u64::from(self.press.0.max(0).unsigned_abs()).min(width);
        let grip_x = along
            .saturating_mul(u64::from(saved_w))
            .checked_div(width)
            .map_or(0, |g| u32::try_from(g).unwrap_or(u32::MAX));
        let right = self.width.saturating_sub(saved_w);
        let x = self.pointer.0.saturating_sub(grip_x).min(right);
        let down = self.press.1.max(0).unsigned_abs();
        let y = self.pointer.1.saturating_sub(down).max(top);
        (x, y)
    }
}

/// Whether a drag of a maximized window has gone far enough to leave it.
pub(in crate::runner) fn past(travel: (i64, i64)) -> bool {
    let limit = u64::from(UNMAXIMIZE_PX);
    travel.0.unsigned_abs() > limit || travel.1.unsigned_abs() > limit
}
