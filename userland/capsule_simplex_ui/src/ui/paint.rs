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

//! The whole window: the chat list on the left, the open chat on the right.
//! The frame around it is the skeleton's, as for every app.

use nonos_app_skeleton::PaintBuffer;

use super::paint_chat::chat;
use super::paint_list::list;
use super::view::{Span, View};

pub fn paint(fb: &mut PaintBuffer, v: &View) {
    let (w, h) = (fb.width, fb.height);
    let list_w = v.m.list_w(w);
    list(fb, v, list_w, h);
    fb.fill_rect(list_w, 0, 1, h, v.c.rule);
    chat(fb, v, Span { x: list_w + 1, w: w.saturating_sub(list_w + 1) }, h);
}
