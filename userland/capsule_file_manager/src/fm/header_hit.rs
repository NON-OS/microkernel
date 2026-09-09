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

extern crate alloc;

use alloc::vec::Vec;

use super::crumbs::crumb_slots;
use super::header_layout::{tool_slots, tool_y};
use super::header_slots::{HeadHit, Slot, CRUMB_H, CRUMB_Y, TOOL_H};
use super::layout::HEADER_H;
use super::state::State;

/// Which header control `(x, y)` lands on. Both bands are tested against the
/// same slot lists the painter drew from, so nothing is clickable off-target.
pub fn head_hit(state: &State, x: u32, y: u32) -> Option<HeadHit> {
    if y >= HEADER_H {
        return None;
    }
    let band = tool_y();
    if y >= band && y < band + TOOL_H {
        if let Some(hit) = pick(tool_slots(state), x) {
            return Some(hit);
        }
    }
    if y >= CRUMB_Y && y < CRUMB_Y + CRUMB_H {
        return pick(crumb_slots(state), x);
    }
    None
}

fn pick(slots: Vec<Slot>, x: u32) -> Option<HeadHit> {
    slots.into_iter().find(|s| x >= s.x && x < s.x + s.w).map(|s| s.hit)
}
