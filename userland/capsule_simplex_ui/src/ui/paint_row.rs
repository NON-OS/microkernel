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

//! One row of the chat list: the name, the last message, its time and the
//! unread count.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::{measure_ttf, PaintBuffer};

use super::badge::badge;
use super::fit::ellipsize;
use super::metrics::{Metrics, NAME_PX, PAD, SMALL_PX};
use super::theme::Colors;
use crate::model::chat::Chat;
use crate::proto::item::clock;

pub struct Row {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub selected: bool,
}

pub fn row(fb: &mut PaintBuffer, r: &Row, chat: &Chat, m: Metrics, c: &Colors) {
    if r.selected {
        fb.fill_rect(r.x, r.y, r.w, r.h, c.selected);
        fb.fill_rect(r.x, r.y, m.px(3), r.h, c.accent);
    }
    let pad = m.px(PAD);
    let right = r.x + r.w - pad;
    let last = chat.last();
    let time = last.map(|i| clock(&i.ts)).unwrap_or("");
    let time_w = measure_ttf(time, SMALL_PX).max(0) as u32;
    fb.text_ttf(
        right.saturating_sub(time_w) as i32,
        (r.y + m.px(12)) as i32,
        time,
        c.quiet,
        SMALL_PX,
    );
    let name_w = r.w.saturating_sub(3 * pad + time_w);
    let name = ellipsize(&chat.name, NAME_PX, name_w);
    fb.text_ttf((r.x + pad) as i32, (r.y + m.px(10)) as i32, &name, c.text, NAME_PX);
    let mut text_w = r.w.saturating_sub(2 * pad);
    if chat.unread > 0 {
        let used = badge(fb, right, r.y + m.px(34), chat.unread, m, c) + m.px(8);
        text_w = text_w.saturating_sub(used);
    }
    let preview = match last {
        Some(i) if i.sent => format!("you: {}", i.text),
        Some(i) => i.text.clone(),
        None => String::from("no messages yet"),
    };
    let preview = ellipsize(&preview, SMALL_PX + 1.0, text_w);
    fb.text_ttf((r.x + pad) as i32, (r.y + m.px(35)) as i32, &preview, c.muted, SMALL_PX + 1.0);
    fb.fill_rect(r.x + pad, r.y + r.h - 1, r.w.saturating_sub(2 * pad), 1, c.rule);
}
