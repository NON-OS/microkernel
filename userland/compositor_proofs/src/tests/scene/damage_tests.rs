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

//! The one property the whole "only repaint what changed" scheme rests on: the
//! rectangles the accumulator hands back must cover every pixel that was ever
//! accumulated. If a merge ever dropped coverage, the compositor would leave
//! stale pixels on screen — exactly the cursor-trail class of bug. These tests
//! drive the real accumulator and check coverage at pixel granularity.

use super::damage_cover::*;
use crate::damage::Rect;

#[test]
fn two_separate_rects_are_both_covered() {
    // The cursor-move case: an old position and a far new position.
    assert_covers(&[
        Rect { x: 10, y: 10, width: 20, height: 20 },
        Rect { x: 400, y: 300, width: 20, height: 20 },
    ]);
}

#[test]
fn touching_rects_merge_without_losing_coverage() {
    assert_covers(&[
        Rect { x: 0, y: 0, width: 16, height: 16 },
        Rect { x: 16, y: 0, width: 16, height: 16 },
        Rect { x: 0, y: 16, width: 16, height: 16 },
    ]);
}

#[test]
fn overflow_past_capacity_still_covers_everything() {
    // More than MAX_RECTS distinct regions forces the smallest-growth merge;
    // coverage must survive it. Twelve well-separated 8x8 squares.
    let mut inputs = Vec::new();
    for i in 0..12u32 {
        inputs.push(Rect { x: i * 40, y: i * 30, width: 8, height: 8 });
    }
    assert_covers(&inputs);
}
