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
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::derive::mix;
use nonos_toolkit::theme::{roles, Opaque};
use nonos_toolkit::widgets::draw::pressed;

use super::bits_num::NUM;
use crate::calc::event::hover::look;
use crate::calc::hit::Hit;
use crate::calc::prog::BITS;
use crate::calc::state::State;
use crate::calc::ui::bits_geom::{cell, BIT_H, LABEL_H};
use crate::calc::ui::metrics::{PX_MONO, R_KEY};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let win_w = fb.width as i32;
    let (r, lh) = (roles(), line_height(PX_MONO).max(1));
    for bit in 0..BITS {
        let c = cell(win_w, bit);
        if c.w == 0 {
            return;
        }
        let set = state.prog & (1i64 << bit) != 0;
        let (bg, ink) = if set { (lit(), r.accent) } else { (r.raised, r.muted) };
        let st = look(state, Hit::Bit(bit));
        let bg = if st.pressed { pressed(bg, ink) } else { bg };
        let radius = R_KEY as u32;
        fb.fill_round(c.x, c.y, c.w, c.h, radius, bg.argb());
        fb.stroke_round(c.x, c.y, c.w, c.h, radius, 1, r.border.argb());
        if st.hovered {
            fb.blend_rect(c.x, c.y, c.w, c.h, r.hover_wash.argb());
            fb.stroke_round(c.x, c.y, c.w, c.h, radius, 1, r.accent_edge.argb());
        }
        let (x, y, w, h) = (c.x as i32, c.y as i32, c.w as i32, c.h as i32);
        let glyph = if set { "1" } else { "0" };
        let gx = x + (w - fb.measure_ttf_mono(glyph, PX_MONO)) / 2;
        fb.text_ttf_mono(gx, y + (h - lh) / 2, glyph, ink.argb(), PX_MONO);
        let name = NUM[bit as usize];
        let nx = x + (w - fb.measure_ttf_mono(name, PX_MONO)) / 2;
        fb.text_ttf_mono(nx, y + BIT_H + (LABEL_H - lh) / 2, name, r.quiet.argb(), PX_MONO);
    }
}

/// The body of a set bit: the key body a step towards the accent.
fn lit() -> Opaque {
    let r = roles();
    Opaque::new(mix(r.raised.argb(), r.accent.argb(), 32))
}
