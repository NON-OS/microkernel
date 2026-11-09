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

use super::list_geom::PILL_GAP;
use super::settings_cells::px;

/// How many of the pills between the first and the last of `widths` fit
/// in a row `room` wide beside those two, which always stay.
pub fn middle_pills(widths: &[i32], room: i32) -> usize {
    let [first, .., last] = widths else {
        return 0;
    };
    let gap = px(PILL_GAP);
    let mut used = first + gap + last;
    let fits = |w: &&i32| {
        used += **w + gap;
        used <= room
    };
    widths[1..widths.len() - 1].iter().take_while(fits).count()
}

/// How far a page `content_h` tall can scroll in a view `view_h` tall.
pub fn max_scroll(content_h: i32, view_h: i32) -> i32 {
    (content_h - view_h).max(0)
}
