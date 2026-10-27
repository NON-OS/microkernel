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

//! The right pane's header: the open chat's name, what carries the frames,
//! and under it the last thing the core said that is not a message.

use nonos_app_skeleton::PaintBuffer;

use super::bubble::width;
use super::fit::ellipsize;
use super::metrics::{HEADER_H, PAD, SMALL_PX, TITLE_PX};
use super::view::{Span, View};
use crate::model::focus::Conn;

pub fn header(fb: &mut PaintBuffer, v: &View, s: Span) {
    let (m, c, model) = (v.m, &v.c, v.model);
    let pad = m.px(PAD);
    let bottom = m.px(HEADER_H);
    fb.fill_rect(s.x, 0, s.w, bottom, c.ground);
    fb.fill_rect(s.x, bottom - 1, s.w, 1, c.rule);
    let carrier = match model.conn {
        Conn::Offline => "no core",
        Conn::Waiting => "waiting for the core",
        Conn::Ready => v.label,
    };
    let cw = width(carrier, SMALL_PX);
    let right = (s.x + s.w).saturating_sub(pad + cw);
    fb.text_ttf(right as i32, m.px(20) as i32, carrier, c.quiet, SMALL_PX);
    let name = model.current().map_or("SimpleX", |chat| chat.name.as_str());
    let name = ellipsize(name, TITLE_PX, right.saturating_sub(s.x + 2 * pad));
    fb.text_ttf((s.x + pad) as i32, m.px(15) as i32, &name, c.text, TITLE_PX);
    if !model.notice.is_empty() {
        let line = ellipsize(&model.notice, SMALL_PX, s.w.saturating_sub(2 * pad));
        fb.text_ttf((s.x + pad) as i32, (bottom + m.px(6)) as i32, &line, c.muted, SMALL_PX);
    }
}
