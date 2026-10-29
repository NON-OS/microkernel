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

//! A one-line text field: the search and the compose line. When the text is
//! wider than the field its start scrolls out, so the caret stays in view.

use nonos_app_skeleton::{measure_ttf, PaintBuffer};

use super::metrics::{Metrics, BODY_PX, RADIUS};
use super::theme::Colors;

pub struct Field {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub focused: bool,
}

/// The longest tail of `text` no wider than `max`.
fn tail(text: &str, max: u32) -> &str {
    let mut start = 0;
    while measure_ttf(&text[start..], BODY_PX).max(0) as u32 > max {
        start += text[start..].chars().next().map_or(1, char::len_utf8);
    }
    &text[start..]
}

pub fn field(fb: &mut PaintBuffer, f: &Field, text: &str, hint: &str, m: Metrics, c: &Colors) {
    let r = m.px(RADIUS).min(f.h / 2);
    fb.fill_round(f.x, f.y, f.w, f.h, r, c.ground);
    let edge = if f.focused { c.accent } else { c.rule };
    fb.stroke_round(f.x, f.y, f.w, f.h, r, 1, edge);
    let inset = m.px(12);
    let ty = (f.y + f.h / 2) as i32 - m.px(9) as i32;
    if text.is_empty() {
        fb.text_ttf((f.x + inset) as i32, ty, hint, c.quiet, BODY_PX);
    }
    let shown = tail(text, f.w.saturating_sub(2 * inset + 2));
    let end = fb.text_ttf((f.x + inset) as i32, ty, shown, c.text, BODY_PX);
    if f.focused {
        let caret_x = if text.is_empty() { (f.x + inset) as i32 } else { end };
        fb.fill_rect(caret_x.max(0) as u32 + 1, f.y + m.px(9), 2, f.h - 2 * m.px(9), c.accent);
    }
}
