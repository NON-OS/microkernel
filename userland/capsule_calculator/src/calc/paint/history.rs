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
use nonos_toolkit::widgets::{text, Card, Ink, Rect, Row, Scrollbar};

use crate::calc::event::hover::look;
use crate::calc::format::{format, DISPLAY_MAX};
use crate::calc::hit::Hit;
use crate::calc::state::State;
use crate::calc::ui::history_geom::{capacity, entry_at, pad, pane, row};
use crate::calc::ui::metrics::metrics;

const EMPTY: &str = "No calculations yet";
/// The scrollbar's width, and how far in from the pane's right edge it
/// starts, in logical pixels.
const THUMB_W: u32 = 3;
const THUMB_IN: u32 = 6;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let p = pane(fb.width as i32, fb.height as i32);
    if p.w <= (pad() * 2) as u32 || p.h == 0 {
        return;
    }
    Card::new().draw(fb, p);
    if state.history.is_empty() {
        let body = metrics().px_body;
        let top = text::centre_top(p.y, p.h, body);
        text::centre(fb, p, top, EMPTY, roles().quiet, body);
        return;
    }
    let rows = capacity(fb.height as i32);
    let mut buf = [0u8; DISPLAY_MAX];
    for i in 0..rows {
        if let Some(entry) = state.history.get(entry_at(state.history_scroll, i)) {
            let n = format(entry.value, 0, &mut buf);
            let value = core::str::from_utf8(&buf[..n]).unwrap_or("");
            let line = Row::new(entry.text()).value(value).value_ink(Ink::Text);
            line.state(look(state, Hit::Row(i))).draw(fb, row(fb.width as i32, i));
        }
    }
    thumb(state, fb, p, rows);
}

/// A slim bar on the pane's right edge when the ring holds more than the page
/// shows: where the page is among all of them, and that there is more.
fn thumb(state: &State, fb: &mut PaintBuffer, pane: Rect, rows: usize) {
    let len = state.history.len();
    if rows == 0 || len <= rows {
        return;
    }
    let (s, pad) = (scale(), pad() as u32);
    let h = pane.h.saturating_sub(pad * 2);
    let track = Rect { x: pane.x + pane.w - s.px(THUMB_IN), y: pane.y + pad, w: s.px(THUMB_W), h };
    Scrollbar::new(state.history_scroll as u32, len as u32, rows as u32).draw(fb, track);
}
