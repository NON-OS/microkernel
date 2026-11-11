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

//! Where Home puts its banner's two actions, its shelves, each track's tile
//! and each shelf's "See all", which the painters and the target table read.

use nonos_toolkit::widgets::Button;

use super::super::geometry::Rect;
use super::super::metrics::{line_h, metrics, px};

const HERO_H: i32 = 196;
pub const SEE_ALL: &str = "See all";
const MIN_CARD: i32 = 128;

/// The banner's height at the display scale.
pub fn hero_h() -> i32 {
    px(HERO_H)
}

/// Banner action `i` in the banner `r`.
pub fn action_rect(r: Rect, i: usize) -> Rect {
    let (m, h) = (metrics(), px(42));
    Rect::new(r.x + m.s7 + i as i32 * px(168), r.bottom() - m.s6 - h, px(156), h)
}

pub fn cols(r: Rect) -> i32 {
    let gap = metrics().s4;
    ((r.w + gap) / (px(MIN_CARD) + gap)).clamp(2, 6)
}

fn card_w(r: Rect) -> i32 {
    (r.w - metrics().s4 * (cols(r) - 1)) / cols(r)
}

fn grid_top(r: Rect) -> i32 {
    let m = metrics();
    r.y + hero_h() + m.s6 + line_h(m.section) + m.s3
}

fn shelf_h(r: Rect) -> i32 {
    let m = metrics();
    (card_w(r) + m.s3 + line_h(m.item) + line_h(m.secondary)).min(r.bottom() - grid_top(r))
}

fn shelf_top(r: Rect, shelf: usize) -> i32 {
    let m = metrics();
    grid_top(r) + shelf as i32 * (shelf_h(r) + m.s6 + line_h(m.section) + m.s3)
}

pub fn shelves(r: Rect) -> usize {
    (0..3).take_while(|&n| shelf_top(r, n) + shelf_h(r) <= r.bottom()).count()
}

pub fn card_rect(r: Rect, shelf: usize, col: i32) -> Rect {
    let w = card_w(r);
    Rect::new(r.x + col * (w + metrics().s4), shelf_top(r, shelf), w, shelf_h(r))
}

pub fn shelf_head(r: Rect, shelf: usize) -> Rect {
    let m = metrics();
    Rect::new(r.x, shelf_top(r, shelf) - line_h(m.section) - m.s3, r.w, line_h(m.section))
}

pub fn see_all_rect(r: Rect, shelf: usize) -> Rect {
    let (h, w) = (shelf_head(r, shelf), Button::new(SEE_ALL).size().0 as i32);
    Rect::new(h.right() + metrics().s2 - w, h.y, w, h.h)
}

pub fn shelf_filled(r: Rect, n: usize, shelf: usize) -> bool {
    shelf * (cols(r) as usize) < n
}
