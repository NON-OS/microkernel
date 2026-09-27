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

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

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
    let mut row = String::new();
    for word in text.split(' ') {
        let trial =
            if row.is_empty() { String::from(word) } else { alloc::format!("{row} {word}") };
        if !row.is_empty() && width(role, &trial) > w {
            draw_in(fb, x, y, role, &row, argb);
            y += step;
            row = String::from(word);
        } else {
            row = trial;
        }
    }
    if !row.is_empty() {
        draw_in(fb, x, y, role, &row, argb);
        y += step;
    }
    y - top
}
