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
//! The right pane while the core's first start waits for the person's
//! name: what it is for, and its field, empty until typed.

use nonos_app_skeleton::PaintBuffer;

use super::metrics::{BODY_PX, COMPOSE_H, HEADER_H, PAD, SMALL_PX, TITLE_PX};
use super::paint_field::{field, Field};
use super::view::{Span, View};
use crate::model::focus::Focus;

const WIDE: u32 = 440;

pub fn name(fb: &mut PaintBuffer, v: &View, s: Span, h: u32) {
    let (m, c, model) = (v.m, &v.c, v.model);
    let (x, mut y) = (s.x + m.px(PAD) * 2, m.px(HEADER_H));
    fb.fill_rect(s.x, 0, s.w, h, c.ground);
    fb.text_ttf(x as i32, y as i32, "Your name in SimpleX", c.text, TITLE_PX);
    y += m.px(34);
    let note = "Your contacts see this name. Enter starts SimpleX with it.";
    fb.text_ttf(x as i32, y as i32, note, c.muted, BODY_PX);
    y += m.px(30);
    let focused = model.focus == Focus::Name;
    let w = m.px(WIDE).min(s.w.saturating_sub(4 * m.px(PAD)));
    field(fb, &Field { x, y, w, h: m.px(COMPOSE_H), focused }, &model.name, "name", m, c);
    y += m.px(COMPOSE_H) + m.px(14);
    if !model.notice.is_empty() {
        fb.text_ttf(x as i32, y as i32, &model.notice, c.quiet, SMALL_PX);
    }
}
