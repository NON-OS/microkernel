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

//! Where the overlay sits.
//!
//! Two columns rather than one long list. Eighteen bindings stacked vertically
//! run past the bottom of a short window, and a help panel that needs scrolling
//! to answer "what does k do" has failed at the one thing it is for.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

pub const PANEL_W: u32 = 660;
pub const PANEL_H: u32 = 470;
pub const ROW_H: u32 = 24;
pub const GROUP_GAP: u32 = 14;

/// The panel, centred. Clamped so a window smaller than the panel still shows
/// its top-left corner rather than placing it off-screen entirely.
pub fn panel(fb_w: u32, fb_h: u32) -> (u32, u32, u32, u32) {
    let s = scale();
    let (w, h) = (s.px(PANEL_W).min(fb_w), s.px(PANEL_H).min(fb_h));

    (fb_w.saturating_sub(w) / 2, fb_h.saturating_sub(h) / 2, w, h)
}

/// A column's box: half the card's content across, left first. Its rows pad
/// themselves.
pub fn column(content: Rect, col: u32) -> Rect {
    let w = content.w / 2;
    Rect { x: content.x + col * w, w, ..content }
}
