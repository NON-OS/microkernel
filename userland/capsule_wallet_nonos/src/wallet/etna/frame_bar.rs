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

//! The fixed parts of the frame: the bar at the top with the way back and
//! the screen's number and name, and the foot with the pinned actions and
//! the status line.

use nonos_app_skeleton::PaintBuffer;

use super::frame_foot::foot_height;
use super::frame_spec::FrameSpec;
use super::parts::label::screen_label;
use super::parts::rule::rule;
use super::rect::Rect;
use super::roles::Role;
use super::symbol::{symbol, Symbol};
use super::text::line;
use super::tokens::{BACK, BANNER_H, BAR_H, COLUMN, HALF, SIDE, TEXT_3};

pub fn bar(fb: &mut PaintBuffer, x0: u32, spec: &FrameSpec) -> Option<Rect> {
    let mut label_x = x0 + SIDE;
    let mut back = None;
    if spec.back {
        let at = Rect::new(x0 + HALF, (BAR_H - BACK) / 2, BACK, BACK);
        symbol(fb, (at.x + BACK / 2) as i32, (at.y + BACK / 2) as i32, Symbol::ChevronLeft, TEXT_3);
        label_x = at.x + BACK;
        back = Some(at);
    }
    let ly = (BAR_H - line(Role::ScreenLabel) as u32) / 2;
    screen_label(fb, label_x as i32, ly as i32, spec.number, spec.title);
    rule(fb, x0, BAR_H, COLUMN, 0);
    back
}

/// The photograph's height here: the phones' full 1290:860, or on a window
/// too short for it, half of what lies between the bar and the foot.
pub fn band_height(fb: &PaintBuffer, spec: &FrameSpec) -> u32 {
    let open = fb.height.saturating_sub(BAR_H + 1 + foot_height(spec));
    BANNER_H.min(open / 2)
}
