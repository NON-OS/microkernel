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

//! The one frame every wallet screen sits in, as ScreenFrame.swift draws
//! it: the bar with the way back and the screen's number and name, the
//! section photograph, the content, the actions pinned at the foot where a
//! hand reaches, and the machine's status line.

use nonos_app_skeleton::PaintBuffer;

use super::band::band;
use super::frame_spec::{FrameLayout, FrameSpec};
use super::parts::action::action;
use super::parts::failure::failure;
use super::parts::label::screen_label;
use super::parts::rule::rule;
use super::parts::status::status_line;
use super::rect::Rect;
use super::roles::Role;
use super::symbol::{symbol, Symbol};
use super::text::line;
use super::tokens::{
    BACK, BAR_H, COLUMN, GAP, HALF, INK, ROOM, SIDE, STATUS_H, TALL, TEXT_3, TIGHT,
};

pub fn frame(fb: &mut PaintBuffer, spec: &FrameSpec) -> FrameLayout {
    let mut out = FrameLayout::default();
    fb.fill_rect(0, 0, fb.width, fb.height, INK);
    let x0 = fb.width.saturating_sub(COLUMN) / 2;
    let mut label_x = x0 + SIDE;
    if spec.back {
        let at = Rect::new(x0 + HALF, (BAR_H - BACK) / 2, BACK, BACK);
        symbol(fb, (at.x + BACK / 2) as i32, (at.y + BACK / 2) as i32, Symbol::ChevronLeft, TEXT_3);
        out.back = Some(at);
        label_x = at.x + BACK;
    }
    let ly = (BAR_H - line(Role::ScreenLabel) as u32) / 2;
    screen_label(fb, label_x as i32, ly as i32, spec.number, spec.title);
    rule(fb, x0, BAR_H, COLUMN, 0);
    let mut y = BAR_H + 1;
    if let Some(which) = spec.backdrop {
        y += band(fb, x0, y, which);
    }
    y += ROOM;
    let cx = x0 + SIDE;
    let cw = COLUMN - 2 * SIDE;
    if let Some(text) = spec.failure {
        let (h, dismiss) = failure(fb, cx, y, cw, text);
        out.dismiss = Some(dismiss);
        y += h + GAP;
    }
    let mut bottom = fb.height;
    if !spec.status.is_empty() {
        bottom -= STATUS_H;
        rule(fb, x0, bottom, COLUMN, 0);
        status_line(fb, x0, bottom + 1, spec.status);
    }
    let n = spec.footer.len() as u32;
    if n > 0 {
        let block = ROOM * 2 + TALL * n + TIGHT * (n - 1);
        bottom -= block + 1;
        rule(fb, x0, bottom, COLUMN, 0);
        for (i, (title, weight, enabled)) in spec.footer.iter().enumerate() {
            let at = Rect::new(cx, bottom + 1 + ROOM + (TALL + TIGHT) * i as u32, cw, TALL);
            action(fb, at, title, *weight, *enabled);
            out.footer[i.min(2)] = at;
        }
    }
    out.content = Rect::new(cx, y, cw, bottom.saturating_sub(y + ROOM));
    out
}
