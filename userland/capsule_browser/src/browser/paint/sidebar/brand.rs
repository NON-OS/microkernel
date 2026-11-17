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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::font::ttf::{builtin_face, draw_text_with, line_height};
use nonos_toolkit::theme::roles;

use crate::browser::ui::rect::Rect;

const LOGO: u32 = 32;
const LOGO_RADIUS: u32 = 9;
const LOGO_PX: f32 = 19.0;
const NAME_PX: f32 = 20.0;
const NAME_GAP: u32 = 12;

/// The Ø mark and the NØNOS wordmark, in bold, centred on the brand row.
pub fn paint(r: Rect, fb: &mut PaintBuffer) {
    let s = scale();
    let (logo, mark_px, name_px) = (s.px(LOGO) as i32, s.font(LOGO_PX), s.font(NAME_PX));
    if r.w < logo || r.h <= 0 {
        return;
    }
    let (ly, role) = (r.y + (r.h - logo) / 2, roles());
    let (side, radius) = (logo as u32, s.px(LOGO_RADIUS));
    fb.fill_round(r.x as u32, ly as u32, side, side, radius, role.ok.argb());
    let mark = "\u{d8}";
    let mx = r.x + (logo - measure_ttf(mark, mark_px)) / 2;
    bold(fb, mx, ly + (logo - line_height(mark_px)) / 2, mark, role.on_accent.argb(), mark_px);
    let (nx, ny) = (r.x + logo + s.px(NAME_GAP) as i32, r.y + (r.h - line_height(name_px)) / 2);
    bold(fb, nx, ny, "N\u{d8}NOS", role.text.argb(), name_px);
}

fn bold(fb: &mut PaintBuffer, x: i32, y: i32, s: &str, argb: u32, px: f32) {
    match builtin_face(false, true) {
        Some(face) => {
            let stride = fb.stride_words as usize;
            let (w, h) = (fb.width, fb.height);
            draw_text_with(face, fb.pixels, stride, w, h, x, y, s, argb, px);
        }
        None => {
            fb.text_ttf(x, y, s, argb, px);
        }
    }
}
