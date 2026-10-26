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
//! The frame's measures at one to one (the design notes, Window
//! Where the three window controls sit: on the right of the title bar,
//! minimize, maximize, then close at the edge, each a whole target.

use super::frame_rect::{frame_rect_at, titlebar_h_at};
use super::hit_test::DecorationHit;
use super::metrics::{CONTROL_H, CONTROL_INSET, CONTROL_W};
use super::rect::Rect;
use super::scale::{at, ONE};

/// The controls, left to right.
pub const CONTROLS: [DecorationHit; 3] =
    [DecorationHit::MinimizeButton, DecorationHit::MaximizeButton, DecorationHit::CloseButton];

pub fn control_rect(i: u32, w: u32, h: u32, maximized: bool) -> Rect {
    control_rect_at(i, w, h, maximized, ONE)
}

/// Control `i` of `CONTROLS` at `quarters` of display scale. The step is
/// scaled as one length, so the targets tile without a rounding gap.
pub fn control_rect_at(i: u32, w: u32, h: u32, maximized: bool, quarters: u32) -> Rect {
    let f = frame_rect_at(w, h, maximized, quarters);
    let (cw, ch) = (at(CONTROL_W, quarters), at(CONTROL_H, quarters));
    let right = (f.x + f.w).saturating_sub(at(CONTROL_INSET, quarters));
    let n = CONTROLS.len() as u32;
    Rect {
        x: right.saturating_sub(cw * (n - i.min(n - 1))),
        y: f.y + titlebar_h_at(quarters).saturating_sub(ch) / 2,
        w: cw,
        h: ch,
    }
}

/// The left edge of the controls: nothing else in the bar reaches past it.
pub fn controls_left_at(w: u32, h: u32, maximized: bool, quarters: u32) -> u32 {
    control_rect_at(0, w, h, maximized, quarters).x
}
