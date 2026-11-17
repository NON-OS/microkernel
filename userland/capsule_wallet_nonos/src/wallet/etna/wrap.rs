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

//! Text set in a column: words broken onto lines no wider than the column,
//! for the lead sentence and every other run of prose.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::text::wrap_by;

use super::roles::Role;
use super::text::{draw_in, line, width};

/// Draw `text` wrapped to `w` and return the height it took.
pub fn wrapped(
    fb: &mut PaintBuffer,
    x: i32,
    top: i32,
    w: i32,
    role: Role,
    text: &str,
    argb: u32,
) -> i32 {
    let step = line(role);
    let mut y = top;
    for (start, end) in lines(w, role, text) {
        draw_in(fb, x, y, role, &text[start..end], argb);
        y += step;
    }
    y - top
}

/// The height `text` wrapped to `w` would take, drawing nothing.
pub fn wrapped_height(w: i32, role: Role, text: &str) -> i32 {
    lines(w, role, text).len() as i32 * line(role)
}

fn lines(w: i32, role: Role, text: &str) -> Vec<(usize, usize)> {
    wrap_by(text, w, |t| width(role, t))
}
