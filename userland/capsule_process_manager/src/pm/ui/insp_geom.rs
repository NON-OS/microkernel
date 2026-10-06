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

use super::metrics::{INSPECTOR_PAD, INSPECTOR_W, INSP_BTN_BOTTOM, INSP_BTN_H};

pub fn pane_x(fb_w: u32) -> u32 {
    fb_w.saturating_sub(INSPECTOR_W)
}

pub fn content_x(fb_w: u32) -> u32 {
    pane_x(fb_w) + INSPECTOR_PAD
}

pub fn content_w() -> u32 {
    INSPECTOR_W.saturating_sub(INSPECTOR_PAD * 2)
}

// The one action, End Process, sits at the bottom of the pane, clear of the
// status strip. The painter and the hit test both read the rect from here
// rather than each deriving one, so a click lands on what was drawn.
pub fn btn(fb_w: u32, fb_h: u32) -> (u32, u32, u32, u32) {
    let y = fb_h.saturating_sub(INSP_BTN_BOTTOM + INSP_BTN_H);
    (content_x(fb_w), y, content_w(), INSP_BTN_H)
}

pub fn btn_at(fb_w: u32, fb_h: u32, x: i32, y: i32) -> bool {
    let (bx, by, bw, bh) = btn(fb_w, fb_h);
    x >= bx as i32 && x < (bx + bw) as i32 && y >= by as i32 && y < (by + bh) as i32
}
