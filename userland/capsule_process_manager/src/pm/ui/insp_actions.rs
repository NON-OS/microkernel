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

use nonos_app_skeleton::PaintBuffer;

use crate::pm::theme::{DANGER, MUTED, SIDEBAR_LINE, TITLE};

use super::insp_geom::btn;
use super::metrics::{BODY_PX, INSP_BTN_RADIUS};
use super::text;

// End Process ends the process at once: the kernel runs no handler for the
// signal, so there is no gentler and no harder way to offer. A process this
// monitor will not end gets no button that looks as if it would: a quiet
// outline says so, and end_selected still refuses.
pub fn paint(fb: &mut PaintBuffer, protected: bool) {
    if protected {
        quiet(fb, b"Protected: cannot be ended here");
        return;
    }
    button(fb, b"End Process");
}

fn quiet(fb: &mut PaintBuffer, label: &[u8]) {
    let (x, y, w, h) = btn(fb.width, fb.height);
    fb.stroke_round(x, y, w, h, INSP_BTN_RADIUS, 1, SIDEBAR_LINE);
    let top = text::centred_top(y, h, BODY_PX);
    let cx = x + w.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
    text::left(fb, cx, top, label, MUTED, BODY_PX);
}

fn button(fb: &mut PaintBuffer, label: &[u8]) {
    let (x, y, w, h) = btn(fb.width, fb.height);
    fb.fill_round(x, y, w, h, INSP_BTN_RADIUS, DANGER);
    fb.stroke_round(x, y, w, h, INSP_BTN_RADIUS, 1, DANGER);
    let top = text::centred_top(y, h, BODY_PX);
    let cx = x + w.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
    text::left(fb, cx, top, label, TITLE, BODY_PX);
}
