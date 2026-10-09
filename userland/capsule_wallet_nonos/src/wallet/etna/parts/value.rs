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

//! A value shown to be read off the screen, an address or a hash: grouped in
//! runs of eight inside a hairline box, with its length under it so a reader
//! knows they have all of it.

use nonos_app_skeleton::PaintBuffer;

use super::super::groups::grouped;
use super::super::rect::Rect;
use super::super::roles::Role;
use super::super::text::{draw_in, line};
use super::super::tokens::{CORNER, OUTLINE, TEXT, TEXT_3};
use super::super::wrap::wrapped;

const PAD: u32 = 16;
/// Lines of a value sit 6 apart, then 10 before the count.
const LEADING: i32 = 6;
const BEFORE_COUNT: i32 = 10;

/// Draw the box at `x, y` across `w`; returns its height.
pub fn value_block(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, value: &str) -> u32 {
    let inner = (w - 2 * PAD) as i32;
    let top = (y + PAD) as i32;
    let body = wrapped(fb, (x + PAD) as i32, top, inner, Role::Fact, &grouped(value), TEXT);
    let lines = (body / line(Role::Fact).max(1)).max(1);
    let body = body + LEADING * (lines - 1);
    let count = alloc::format!("{} characters", value.chars().count());
    draw_in(fb, (x + PAD) as i32, top + body + BEFORE_COUNT, Role::Fact, &count, TEXT_3);
    let h = PAD * 2 + (body + BEFORE_COUNT + line(Role::Fact)) as u32;
    let at = Rect::new(x, y, w, h);
    fb.stroke_round(at.x, at.y, at.w, at.h, CORNER, 1, OUTLINE);
    h
}
