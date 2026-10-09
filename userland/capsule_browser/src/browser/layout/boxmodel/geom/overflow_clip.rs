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

use crate::browser::css::Computed;

/// The clip a box's overflow puts on its content, [x0, y0, x1, y1] in page
/// coordinates for a border box at `r` ([x, y, w]) with definite height
/// `h`. Horizontal clipping applies whenever the box clips that axis; the
/// vertical only with a definite height, since an auto box grows to hold
/// its content anyway. None when nothing is clipped.
pub(crate) fn overflow_clip(s: &Computed, r: [i32; 3], h: Option<i32>) -> Option<[i32; 4]> {
    let [x, y, w] = r;
    let open = (i32::MIN, i32::MAX);
    let (x0, x1) = match s.clips_x() {
        true => (x + s.border_left as i32, x + w - s.border_right as i32),
        false => open,
    };
    let (y0, y1) = match h {
        Some(h) if s.clips_y() => (y + s.border_top as i32, y + h - s.border_bottom as i32),
        _ => open,
    };
    ((x0, x1, y0, y1) != (open.0, open.1, open.0, open.1)).then_some([x0, y0, x1, y1])
}
