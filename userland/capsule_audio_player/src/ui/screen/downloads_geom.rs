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

//! Where the Downloads page puts Clear finished, its rows and each row's
//! buttons, which the painter and the target table both read.

use nonos_toolkit::widgets::text;

use super::super::geometry::Rect;
use super::super::metrics::{line_h, metrics, px};

pub const CLEAR: &str = "Clear finished";
const ROW_H: i32 = 84;
const BTN_H: i32 = 36;

fn list_top(r: Rect) -> i32 {
    let m = metrics();
    r.y + line_h(m.page) + line_h(m.secondary) + m.s6
}

/// Rows the page has room for.
pub fn visible(r: Rect) -> usize {
    ((r.bottom() - list_top(r)).max(0) / (px(ROW_H) + metrics().s3)) as usize
}

pub fn row_rect(r: Rect, slot: usize) -> Rect {
    let h = px(ROW_H);
    Rect::new(r.x, list_top(r) + slot as i32 * (h + metrics().s3), r.w, h)
}

/// Where the empty page's card starts.
pub fn empty_rect(r: Rect) -> Rect {
    Rect::new(r.x, list_top(r), r.w.min(px(760)), px(220))
}

fn button_w(label: &str) -> i32 {
    let m = metrics();
    text::width(label, m.item) as i32 + m.s3 * 4
}

pub fn clear_rect(r: Rect) -> Rect {
    let (w, h) = (button_w(CLEAR), px(38));
    Rect::new(r.right() - w, r.y + (line_h(metrics().page) - h) / 2, w, h)
}

/// The buttons of the row in `rr`, right to left in the order `labels` gives
/// them, so the likeliest sits at the edge; each with its place in `labels`.
pub fn buttons<'a>(rr: Rect, labels: &'a [&str]) -> impl Iterator<Item = (usize, Rect)> + 'a {
    let (m, h) = (metrics(), px(BTN_H));
    let mut right = rr.right() - m.s4;
    labels.iter().enumerate().map(move |(k, l)| {
        let w = button_w(l);
        right -= w;
        let b = Rect::new(right, rr.cy() - h / 2, w, h);
        right -= m.s3;
        (k, b)
    })
}
