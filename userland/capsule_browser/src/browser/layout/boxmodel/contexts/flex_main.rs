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

use crate::browser::css::{Computed, Justify};

use super::super::edges_y::edges_y;

/* How a line's leftover main space `left` is handed out among `n` items
 * with `gap` between them: (offset of the first item, step after each
 * item, extra for each auto margin). Auto margins take it first; else
 * justify-content does. The space-* forms need room to spread and fall
 * back to the start when the items overflow, as their safe fallbacks do;
 * center and end still move an overflowing line. */
pub(in super::super) fn main_offsets(
    left: i32,
    n: i32,
    autos: i32,
    gap: i32,
    j: Justify,
) -> (i32, i32, i32) {
    if autos > 0 && left > 0 {
        return (0, gap, left / autos);
    }
    let n = n.max(1);
    let room = left.max(0);
    match j {
        Justify::Start => (0, gap, 0),
        Justify::Center => (left / 2, gap, 0),
        Justify::End => (left, gap, 0),
        Justify::Between if n > 1 => (0, gap + room / (n - 1), 0),
        Justify::Between => (0, gap, 0),
        Justify::Around => (room / n / 2, gap + room / n, 0),
        Justify::Evenly => (room / (n + 1), gap + room / (n + 1), 0),
    }
}

/* A px flex-basis as a border-box height; other bases wait for content. */
pub(in super::super) fn basis_h(s: &Computed) -> Option<i32> {
    let b = s.flex_basis.definite_px().filter(|&b| b > 0)?;
    let (t, bo) = edges_y(s);
    Some(if s.border_box { b } else { b + t + bo })
}
