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

use super::super::metrics::CARD_PAD;

pub const PANEL_W: u32 = 660;
pub const PANEL_H: u32 = 470;
pub const TITLE_H: u32 = 44;
pub const ROW_H: u32 = 24;
pub const GROUP_GAP: u32 = 14;
pub const KEY_W: u32 = 74;

/// The panel, centred. Clamped so a window smaller than the panel still shows
/// its top-left corner rather than placing it off-screen entirely.
pub fn panel(fb_w: u32, fb_h: u32) -> (u32, u32, u32, u32) {
    let w = PANEL_W.min(fb_w);
    let h = PANEL_H.min(fb_h);
    (fb_w.saturating_sub(w) / 2, fb_h.saturating_sub(h) / 2, w, h)
}

/// The x of a column and how wide its labels may run.
pub fn column(panel_x: u32, panel_w: u32, col: u32) -> (u32, u32) {
    let inner = panel_w.saturating_sub(CARD_PAD * 2);
    let col_w = inner / 2;
    (panel_x + CARD_PAD + col * col_w, col_w.saturating_sub(CARD_PAD))
}
