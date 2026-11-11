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
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::paint::measure_ttf;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::fit_size_by;
use nonos_toolkit::widgets::{text, Card};

use crate::calc::convert::{convert, list};
use crate::calc::error_kind::ErrorKind;
use crate::calc::format::{error_text, format, DISPLAY_MAX};
use crate::calc::state::State;
use crate::calc::ui::convert_geom::result;
use crate::calc::ui::metrics::{metrics, px};

/// The result's font sizes, largest first, and its inset, in logical pixels.
const LADDER: [f32; 3] = [34.0, 26.0, 17.0];
const PAD: i32 = 14;

fn pair(state: &State, fb: &mut PaintBuffer, x: u32, y: i32, budget: u32) {
    let units = list(state.cat);
    let src = units.get(state.from).map(|unit| unit.name).unwrap_or("");
    let dst = units.get(state.to).map(|unit| unit.name).unwrap_or("");
    let (muted, body) = (roles().muted, metrics().px_body);
    let pen = |at: i32| at.max(0) as u32;
    let mut cursor = text::left(fb, x, y, &text::truncate(src, budget, body), muted, body);
    cursor = text::left(fb, pen(cursor), y, " to ", muted, body);
    text::left(fb, pen(cursor), y, &text::truncate(dst, budget, body), muted, body);
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (at, pad) = (result(fb.width as i32, fb.height as i32), px(PAD));
    if at.w <= (pad * 2) as u32 || at.h == 0 {
        return;
    }
    Card::new().draw(fb, at);
    let (x, y, w, h) = (at.x as i32, at.y as i32, at.w as i32, at.h as i32);
    let r = roles();
    pair(state, fb, at.x + pad as u32, y + pad, at.w / 3);
    let mut buf = [0u8; DISPLAY_MAX];
    let (text, ink) = match convert(state.cat, state.from, state.to, state.display) {
        Some(value) => {
            let n = format(value, 0, &mut buf);
            (core::str::from_utf8(&buf[..n]).unwrap_or("0"), r.accent)
        }
        /* The units are fixed tables, so the only way a conversion fails is a
         * product past what the fixed point holds. */
        None => (error_text(ErrorKind::Overflow), r.danger),
    };
    let ladder = LADDER.map(|px| scale().font(px));
    let px = fit_size_by(text, w / 2, &ladder, measure_ttf).unwrap_or(ladder[ladder.len() - 1]);
    let tx = x + w - pad - fb.measure_ttf(text, px);
    fb.text_ttf(tx, y + (h - line_height(px).max(1)) / 2, text, ink.argb(), px);
}
