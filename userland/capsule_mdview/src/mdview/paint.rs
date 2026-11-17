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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;

use super::draw::{draw_line, margin};
use super::layout::{line_height, Line};
use super::scroll::{place, top};

/// The accent band's height, the error's and the hint's font sizes and how far
/// below the error the hint sits, in logical pixels.
const BAND: u32 = 4;
const ERROR_PX: f32 = 15.0;
const HINT_PX: f32 = 13.0;
const HINT_DROP: u32 = 26;

/// Draws the lines wholly inside the window once the page is scrolled up by
/// `scroll` pixels.
pub fn paint(fb: &mut PaintBuffer, lines: &[Line], error: Option<&'static str>, scroll: u32) {
    let (r, bottom, s) = (roles(), fb.height as i32, scale());
    fb.clear(r.bg.argb());
    fb.fill_rect(0, 0, fb.width, s.px(BAND), r.accent.argb());
    if let Some(message) = error {
        fb.text_ttf(margin(), top(), message, r.danger.argb(), s.font(ERROR_PX));
        let hint_y = top() + s.px(HINT_DROP) as i32;
        fb.text_ttf(margin(), hint_y, "esc closes this window", r.quiet.argb(), s.font(HINT_PX));
        return;
    }
    place(lines, |top, line| {
        let y = top - scroll as i32;
        if y >= 0 && y + line_height(line.style) <= bottom {
            draw_line(fb, line, y);
        }
    });
    /* A line scrolled up to the top edge is drawn under the accent band. */
    fb.fill_rect(0, 0, fb.width, s.px(BAND), r.accent.argb());
}
