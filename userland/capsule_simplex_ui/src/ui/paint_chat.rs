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

//! The right pane: the open chat's items, newest at the bottom, above the
//! compose line and under the header.

use nonos_app_skeleton::PaintBuffer;

use super::bubble::layout;
use super::bubble_draw::draw;
use super::metrics::{BODY_PX, COMPOSE_H, GAP, HEADER_H, PAD};
use super::paint_field::{field, Field};
use super::paint_header::header;
use super::view::{Span, View};
use crate::model::focus::Focus;

const HINT: &str = "message, or /invite, /address, /connect <link>";

fn items(fb: &mut PaintBuffer, v: &View, s: Span, top: u32, bottom: u32) {
    let m = v.m;
    let Some(chat) = v.model.current() else {
        let at = ((s.x + m.px(PAD)) as i32, (top + m.px(PAD)) as i32);
        fb.text_ttf(at.0, at.1, "choose a chat", v.c.muted, BODY_PX);
        return;
    };
    let mut y = bottom;
    for item in chat.items.iter().rev() {
        let b = layout(item, s.w * 7 / 10, m);
        if y < top + b.h {
            break;
        }
        y -= b.h;
        draw(fb, &b, s.x, s.w, y, m, &v.c);
        y = y.saturating_sub(m.px(GAP));
    }
}

pub fn chat(fb: &mut PaintBuffer, v: &View, s: Span, h: u32) {
    let (m, pad) = (v.m, v.m.px(PAD));
    fb.fill_rect(s.x, 0, s.w, h, v.c.ground);
    let compose_y = h.saturating_sub(pad + m.px(COMPOSE_H));
    let floor = compose_y.saturating_sub(m.px(GAP));
    items(fb, v, s, m.px(HEADER_H) + m.px(28), floor);
    header(fb, v, s);
    fb.fill_rect(s.x, floor, s.w, h - floor, v.c.ground);
    let focused = v.model.focus == Focus::Compose;
    let f = Field { x: s.x + pad, y: compose_y, w: s.w - 2 * pad, h: m.px(COMPOSE_H), focused };
    field(fb, &f, &v.model.compose, HINT, m, &v.c);
}
