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

use alloc::string::String;

use super::grid_line_names::Names;
use super::grid_occupy::MAX_SPAN;

enum Tok {
    Auto,
    Line(i16),
    Span(u16),
}

/* One side of an axis as written: auto, span N, a line number (negative
 * ones counted from the end of `explicit` tracks when that is known), or
 * a line name. */
fn tok(t: &Option<String>, names: Names, end: bool, explicit: Option<usize>) -> Tok {
    let Some(t) = t.as_deref().map(str::trim).filter(|t| !t.is_empty() && *t != "auto") else {
        return Tok::Auto;
    };
    let (mut span, mut num, mut name) = (false, None, None);
    for p in t.split_whitespace() {
        match p.parse::<i16>() {
            Ok(n) => num = Some(n),
            Err(_) if p == "span" => span = true,
            Err(_) => name = Some(p),
        }
    }
    match (span, num, name) {
        (true, n, _) => Tok::Span(n.unwrap_or(1).clamp(1, MAX_SPAN as i16) as u16),
        (false, Some(n), None) if n < 0 => match explicit {
            Some(k) => Tok::Line((k.min(i16::MAX as usize - 2) as i16 + 2 + n).max(1)),
            None => Tok::Line(n),
        },
        (false, Some(n), None) if n > 0 => Tok::Line(n),
        (false, _, Some(nm)) => names.line(nm, end).map_or(Tok::Auto, Tok::Line),
        _ => Tok::Auto,
    }
}

/* One axis of a placement: its [start, end] lines (0 for auto) and span. */
pub(in super::super) fn axis(
    a: &Option<String>,
    b: &Option<String>,
    names: Names,
    explicit: Option<usize>,
) -> ([i16; 2], u16) {
    match (tok(a, names, false, explicit), tok(b, names, true, explicit)) {
        (Tok::Line(s), Tok::Line(e)) => ([s, e], 1),
        (Tok::Line(s), Tok::Span(n)) => ([s, 0], n),
        (Tok::Line(s), Tok::Auto) => ([s, 0], 1),
        (Tok::Span(n), Tok::Line(e)) => ([0, e], n),
        (Tok::Span(n), _) | (Tok::Auto, Tok::Span(n)) => ([0, 0], n),
        (Tok::Auto, Tok::Line(e)) => ([0, e], 1),
        (Tok::Auto, Tok::Auto) => ([0, 0], 1),
    }
}
