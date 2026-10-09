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

//! Where the grid motif's floor lines fall: from the horizon down, each gap
//! wider than the last, as a floor recedes. Its own file so the proofs hold
//! it: a gap that started at 0 on a small cover never grew, and the loop that
//! drew them never reached the floor's edge, which held the window for good.

extern crate alloc;

use alloc::vec::Vec;

/// The rows of the floor lines below `horizon` and above `bottom`, for a
/// cover whose shorter side is `side`.
pub fn rungs(horizon: i32, bottom: i32, side: i32) -> Vec<i32> {
    // Wide enough that the gap, which grows by half each time, never wraps.
    let bottom = i64::from(bottom);
    let mut out = Vec::new();
    let mut y = i64::from(horizon);
    let mut step = (i64::from(side) * 3 / 100).max(1);
    loop {
        y += step;
        if y >= bottom {
            return out;
        }
        out.push(y as i32);
        step = (step * 145 / 100).max(step + 1);
    }
}
