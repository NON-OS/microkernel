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

//! A verbatim copy of About's `prose_wrap::split` and the loop `prose` walked
//! it with, kept to pin the toolkit's `wrap_hard` against.

use nonos_toolkit::widgets::text::width;

use crate::about::ui::metrics::metrics;

pub fn valid(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes).unwrap_or("")
}

pub fn split(b: &[u8], w: u32) -> (usize, usize) {
    let mut end = 0usize;
    let mut i = 0usize;
    while i < b.len() {
        let next = match b[i..].iter().position(|c| *c == b' ') {
            Some(p) => i + p,
            None => b.len(),
        };
        if width(valid(&b[..next]), metrics().body_px) > w {
            break;
        }
        end = next;
        i = next + 1;
    }
    if end == 0 {
        let cut = fallback(b, w);
        return (cut, cut);
    }
    (end, (end + 1).min(b.len()))
}

fn fallback(b: &[u8], w: u32) -> usize {
    let s = core::str::from_utf8(b).unwrap_or("");
    let mut end = s.len();
    while end > 1 && width(valid(&b[..end]), metrics().body_px) > w {
        end -= 1;
        while end > 1 && !s.is_char_boundary(end) {
            end -= 1;
        }
    }
    end.max(1)
}

pub fn old_lines(b: &[u8], w: u32) -> Vec<&[u8]> {
    let (mut rest, mut lines) = (b, Vec::new());
    while !rest.is_empty() {
        let (end, next) = split(rest, w);
        lines.push(&rest[..end]);
        rest = &rest[next.max(1)..];
    }
    lines
}
