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

//! Text wrapped at word boundaries to a width in pixels.

use super::style::Style;
use super::text::{draw, metrics, width};

/// Draw `s` in `style`, wrapped to `w`, at most `max` lines from (x, y).
/// Returns the y below the last line drawn.
pub fn draw_wrapped(x: u32, y: u32, w: u32, s: &[u8], style: Style, c: u32, max: u32) -> u32 {
    let line = metrics(style).line;
    let (mut rest, mut y) = (s, y);
    for _ in 0..max {
        if rest.is_empty() {
            break;
        }
        let (head, tail) = split(rest, w, style);
        draw(x, y, head, style, c);
        rest = tail;
        y += line;
    }
    y
}

/// The lines `s` takes at width `w`.
pub fn lines(s: &[u8], w: u32, style: Style) -> u32 {
    let (mut rest, mut n) = (s, 0);
    while !rest.is_empty() {
        rest = split(rest, w, style).1;
        n += 1;
    }
    n
}

/// The longest run of whole words that fits `w`, and the rest after it.
fn split(s: &[u8], w: u32, style: Style) -> (&[u8], &[u8]) {
    let mut cut = 0;
    for (i, &b) in s.iter().enumerate() {
        if b == b' ' {
            if width(&s[..i], style) > w {
                break;
            }
            cut = i;
        }
    }
    if width(s, style) <= w {
        return (s, &[]);
    }
    if cut == 0 {
        cut = s.iter().position(|&b| b == b' ').unwrap_or(s.len());
    }
    let tail = &s[cut..];
    (&s[..cut], tail.strip_prefix(b" ").unwrap_or(tail))
}
