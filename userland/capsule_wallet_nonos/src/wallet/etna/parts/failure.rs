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

//! What the core refused, in one plain sentence on the banner ground, and a
//! way to put it away. It is the first thing in a screen's content.

use nonos_app_skeleton::PaintBuffer;

use super::super::rect::Rect;
use super::super::roles::Role;
use super::super::text::{draw_in, line, width};
use super::super::tokens::{BANNER, RADIUS, TEXT, TEXT_3};
use super::super::wrap::wrapped;

const PAD: u32 = 16;
const GAP: u32 = 8;

/// Draw across `w` at `x, y`; returns the height and where Dismiss is.
pub fn failure(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &str) -> (u32, Rect) {
    let inner = (w - 2 * PAD) as i32;
    // Measured first by drawing off the bottom, then drawn on its ground.
    let body = wrapped(fb, (x + PAD) as i32, i32::MAX / 2, inner, Role::Lead, text, TEXT) as u32;
    let h = PAD * 2 + body + GAP + line(Role::Lead) as u32;
    fb.fill_round(x, y, w, h, RADIUS, BANNER);
    wrapped(fb, (x + PAD) as i32, (y + PAD) as i32, inner, Role::Lead, text, TEXT);
    let dy = y + PAD + body + GAP;
    draw_in(fb, (x + PAD) as i32, dy as i32, Role::Lead, "Dismiss", TEXT_3);
    let dw = width(Role::Lead, "Dismiss") as u32;
    (h, Rect::new(x + PAD, dy, dw, line(Role::Lead) as u32))
}
