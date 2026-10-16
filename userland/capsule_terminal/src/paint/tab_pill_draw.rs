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

//! Painting one tab pill: its fill, status dot, fitted label and close mark.

use nonos_app_skeleton::PaintBuffer;

use super::{close_rect, DOT_X, LABEL_PX, RADIUS, TEXT_X};
use crate::layout::Rect;
use crate::paint::fit_text::{fit_text, width_of};
use crate::paint::tokens::{DOT_IDLE, DOT_OK, TAB_ACTIVE, TAB_ACTIVE_EDGE, TAB_IDLE};
use crate::paint::tokens::{TOOLBAR_ACTIVE, TOOLBAR_LABEL};

pub fn draw_pill(fb: &mut PaintBuffer, r: Rect, label: &[u8], name_len: usize, active: bool) {
    if r.w == 0 {
        return;
    }
    fb.fill_round(r.x, r.y, r.w, r.h, RADIUS, if active { TAB_ACTIVE } else { TAB_IDLE });
    if active {
        fb.stroke_round(r.x, r.y, r.w, r.h, RADIUS, 1, TAB_ACTIVE_EDGE);
    }
    fb.circle(r.x + DOT_X, r.y + r.h / 2, 3, if active { DOT_OK } else { DOT_IDLE });
    let fg = if active { TOOLBAR_ACTIVE } else { TOOLBAR_LABEL };
    let baseline = (r.y + r.h / 2) as i32 - (LABEL_PX as i32) / 2;
    let close = close_rect(r);
    let room = close.x.saturating_sub(r.x + TEXT_X);
    let full = core::str::from_utf8(label).unwrap_or("");
    let cut = if width_of(fb, full, LABEL_PX) <= room {
        full
    } else {
        let name = core::str::from_utf8(&label[..name_len.min(label.len())]).unwrap_or("");
        fit_text(fb, name, LABEL_PX, room)
    };
    fb.text_ttf((r.x + TEXT_X) as i32, baseline, cut, fg, LABEL_PX);
    let cx = close.x + close.w / 2 - width_of(fb, "x", LABEL_PX) / 2;
    fb.text_ttf(cx as i32, baseline, "x", fg, LABEL_PX);
}
