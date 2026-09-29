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

use super::calc::split_top::words;
use super::computed::Size;
use super::parse_px::{parse_margin, parse_px};
use super::parse_size::parse_offset;

/* Expand a 1-4 value box shorthand into [top, right, bottom, left]. Words
 * split at paren depth 0, so a function argument list stays one word. */
fn expand<T: Copy + Default>(
    value: &str,
    mut one: impl FnMut(&str) -> Option<T>,
) -> Option<[T; 4]> {
    let mut vals = [T::default(); 4];
    let mut n = 0;
    for part in words(value) {
        *vals.get_mut(n)? = one(part)?;
        n += 1;
    }
    match n {
        1 => Some([vals[0]; 4]),
        2 => Some([vals[0], vals[1], vals[0], vals[1]]),
        3 => Some([vals[0], vals[1], vals[2], vals[1]]),
        4 => Some(vals),
        _ => None,
    }
}

fn auto(part: &str) -> bool {
    part.eq_ignore_ascii_case("auto")
}

/// Non-negative sides capped at `max` (padding, border widths). "auto"
/// resolves to 0; any unparsable component rejects the whole value.
pub(super) fn sides(value: &str, em: u32, max: u32) -> Option<[u32; 4]> {
    expand(value, |p| if auto(p) { Some(0) } else { Some(parse_px(p, em)?.min(max)) })
}

/// Signed sides for margins as (px, per-mille of the containing width);
/// "auto" resolves to 0 here and is tracked apart.
pub(super) fn signed_sides(value: &str, em: u32) -> Option<[(i32, i32); 4]> {
    expand(value, |p| if auto(p) { Some((0, 0)) } else { parse_margin(p, em) })
}

/// Signed offsets (top, right, bottom, left) from the 1-4 value `inset`
/// shorthand, each a length, percentage or auto.
pub(super) fn offsets(value: &str, em: u32) -> Option<[Size; 4]> {
    let v = expand(value, |p| parse_offset(p, em).map(Some))?;
    Some(v.map(|s| s.unwrap_or(Size::Auto)))
}
