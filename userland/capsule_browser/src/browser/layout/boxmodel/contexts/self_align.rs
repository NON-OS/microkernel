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

use crate::browser::css::Align;

use super::super::display_list::DisplayList;
use super::super::shift_down::shift_down;

/* Line an item up in a cross space `room` tall, laid at its top margin:
 * frags [a, b) are the item's, `mhm` its [top margin, border-box height,
 * bottom margin]. Center and end move it down (up, if it overflows);
 * stretch grows its own box to fill the space, when it is shorter. */
pub(in super::super) fn cross_shift(
    frags: &mut DisplayList,
    ab: [usize; 2],
    mhm: [i32; 3],
    room: i32,
    align: Align,
    clip: Option<[i32; 4]>,
) {
    let ([a, b], [mt, h, mb]) = (ab, mhm);
    let free = room.saturating_sub(mt.saturating_add(h).saturating_add(mb));
    match align {
        Align::Start => {}
        Align::Center => shift_down(frags, a, b, free / 2, clip),
        Align::End => shift_down(frags, a, b, free, clip),
        Align::Stretch => {
            if let Some(f) = frags.get_mut(a).filter(|_| a < b && free > 0) {
                f.h = f.h.max(room - mt - mb);
            }
        }
    }
}

/* Where an item of border-box width `w` sits in a space `room` wide by
 * its justify alignment: the offset from the space's start edge, mirrored
 * for right-to-left. Stretch and start both sit at the start. */
pub(in super::super) fn inline_offset(room: i32, w: i32, align: Align, rtl: bool) -> i32 {
    let free = room.saturating_sub(w);
    let start = match align {
        Align::Center => free / 2,
        Align::End => free,
        Align::Start | Align::Stretch => 0,
    };
    if rtl {
        free - start
    } else {
        start
    }
}
