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

use crate::browser::css::calc::split_top::words;
use crate::browser::layout::boxmodel::Rel;

use super::transform_fn::rel_len;

const CENTER: Rel = (0, 500);

/* One position component: which axis a keyword pins it to (0 either, 1 x,
 * 2 y) and its box-relative offset. */
fn part(w: &str, fs: u32) -> Option<(u8, Rel)> {
    Some(match w.to_ascii_lowercase().as_str() {
        "left" => (1, (0, 0)),
        "right" => (1, (0, 1000)),
        "top" => (2, (0, 0)),
        "bottom" => (2, (0, 1000)),
        "center" => (0, CENTER),
        _ => (0, rel_len(w, fs)?),
    })
}

/// A position as [x, y] box-relative offsets: transform-origin, and the
/// `at` of a clip-path circle or ellipse. One value sets the axis its
/// keyword names and centres the other; two values read x then y unless
/// their keywords say otherwise (`top left`). A third (z) value is ignored.
pub(super) fn parse_position(value: &str, fs: u32) -> Option<[Rel; 2]> {
    let mut it = words(value);
    let a = part(it.next()?, fs)?;
    let Some(bw) = it.next() else {
        return Some(if a.0 == 2 { [CENTER, a.1] } else { [a.1, CENTER] });
    };
    let b = part(bw, fs)?;
    match (a.0, b.0) {
        (2, 0 | 1) | (0, 1) => Some([b.1, a.1]),
        (1, 1) | (2, 2) => None,
        _ => Some([a.1, b.1]),
    }
}
