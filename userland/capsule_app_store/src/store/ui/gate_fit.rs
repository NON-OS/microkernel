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
//! Which gate rows the detail pane draws when they do not all fit above its
//! foot. A failing gate is the reason a listing cannot be installed, so it
//! is never the row left out while a passing one is drawn.

use alloc::vec::Vec;

/// The rows to draw, as indices into `passes` in the order drawn, and how
/// many are left out. Every row, in order, when all fit in `room` rows;
/// else the failing ones first, then the passing, in one row less, which
/// the line saying how many more there are takes.
pub fn shown(passes: &[bool], room: usize) -> (Vec<usize>, usize) {
    if passes.len() <= room {
        return ((0..passes.len()).collect(), 0);
    }
    let fails = (0..passes.len()).filter(|&i| !passes[i]);
    let order: Vec<usize> = fails.chain((0..passes.len()).filter(|&i| passes[i])).collect();
    let take = room.saturating_sub(1).min(order.len());
    (order[..take].to_vec(), order.len() - take)
}
