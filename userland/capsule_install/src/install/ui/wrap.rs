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

//! Word wrap for the few paragraphs this window shows.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::text::{line, width};

/// Word-wrapped lines of `s` within `max_w`, painted from `top` at `step`
/// per line. Returns the y after the last line.
#[allow(clippy::too_many_arguments)]
pub fn paragraph(
    fb: &mut PaintBuffer,
    x: u32,
    top: u32,
    max_w: u32,
    s: &str,
    argb: u32,
    px: f32,
    step: u32,
) -> u32 {
    let mut y = top;
    let mut current = String::new();
    for word in s.split(' ') {
        let candidate = if current.is_empty() {
            String::from(word)
        } else {
            alloc::format!("{current} {word}")
        };
        if width(fb, &candidate, px) > max_w && !current.is_empty() {
            line(fb, x, y, &current, argb, px);
            y += step;
            current = String::from(word);
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        line(fb, x, y, &current, argb, px);
        y += step;
    }
    y
}
