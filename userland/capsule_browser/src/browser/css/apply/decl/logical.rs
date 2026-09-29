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

use crate::browser::css::computed::Computed;

use super::super::margin::apply_margin;
use super::super::padding::apply_padding;

/* A logical margin or padding property (margin-block-start,
 * padding-inline, ...) applied as the physical sides it maps to in
 * horizontal-tb, left-to-right writing, the only mode laid out here. A
 * two-side shorthand takes start then end, one value serving both. None
 * when `name` is not one of them. */
pub(super) fn apply_logical(c: &mut Computed, name: &str, value: &str, fs: u32) -> Option<bool> {
    let (margin, axis) = match name.strip_prefix("margin-") {
        Some(rest) => (true, rest),
        None => (false, name.strip_prefix("padding-")?),
    };
    let sides: (&str, &str) = match axis {
        "block-start" => ("top", ""),
        "block-end" => ("bottom", ""),
        "inline-start" => ("left", ""),
        "inline-end" => ("right", ""),
        "block" => ("top", "bottom"),
        "inline" => ("left", "right"),
        _ => return None,
    };
    let v = value.trim();
    let (first, second) = split_first(v);
    let mut set = |side: &str, v: &str| {
        let full = if margin { ["margin-", side].concat() } else { ["padding-", side].concat() };
        match margin {
            true => apply_margin(c, &full, v, fs),
            false => apply_padding(c, &full, v, fs),
        }
    };
    let mut ok = set(sides.0, if sides.1.is_empty() { v } else { first });
    if !sides.1.is_empty() {
        ok &= set(sides.1, if second.is_empty() { first } else { second });
    }
    Some(ok)
}

/* The first value of `v` and the rest, split at the first space outside
 * parentheses, so calc(1px + 2px) stays whole. */
fn split_first(v: &str) -> (&str, &str) {
    let mut depth = 0u32;
    for (i, b) in v.bytes().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b' ' | b'\t' | b'\n' if depth == 0 => return (&v[..i], v[i..].trim()),
            _ => {}
        }
    }
    (v, "")
}
