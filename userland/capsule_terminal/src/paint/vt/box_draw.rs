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

//! Box-drawing characters stroked across the whole cell.

use nonos_app_skeleton::PaintBuffer;

use super::box_arms::{arms, DOUBLE, HEAVY, NONE};

/// Stroke `ch` over the cell at `(x, y)` sized `w` by `h`. False when it
/// is not a line character, so the caller draws the glyph instead.
pub fn stroke(fb: &mut PaintBuffer, ch: char, x: u32, y: u32, w: u32, h: u32, argb: u32) -> bool {
    let Some([up, down, left, right]) = arms(ch) else { return false };
    let t = (w / 8).max(1);
    let (cx, cy) = (x + w / 2, y + h / 2);
    for (arm, across, first) in
        [(up, false, true), (down, false, false), (left, true, true), (right, true, false)]
    {
        if arm == NONE {
            continue;
        }
        let k = if arm == HEAVY { 2 * t } else { t };
        /*
         * An arm runs from the cell's edge to just past the centre line, by
         * half its own width, so the arms of a cross meet with no notch. A
         * double arm is two strokes either side of the centre.
         */
        let gap = if arm == DOUBLE { t + 1 } else { 0 };
        let past = k / 2 + gap + k % 2;
        let sides: &[i32] = if arm == DOUBLE { &[-1, 1] } else { &[0] };
        for &side in sides {
            let lane = |c: u32| (c as i32 + side * gap as i32) as u32 - k / 2;
            if across {
                let (a, b) = if first { (x, cx + past) } else { (cx - past, x + w) };
                fb.fill_rect(a, lane(cy), b - a, k, argb);
            } else {
                let (a, b) = if first { (y, cy + past) } else { (cy - past, y + h) };
                fb.fill_rect(lane(cx), a, k, b - a, argb);
            }
        }
    }
    true
}
