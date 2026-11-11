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

use super::chip::chip_h;
use super::info_geom::InfoGeom;
use super::info_tag_add::add_box;
use super::info_tag_geom::{remove_x, tag_slots, X_BOX};
use super::layout::px;
use super::state::State;
use super::target::Target;

/// Tested against the same wrapped pass `info_tags` drew, so the box that
/// removes a tag is exactly the cross the user aimed at. A tag is named by its
/// index in the cursor path's tags, which the click reads the name back from.
pub fn tag_aim(state: &State, g: &InfoGeom, x: u32, y: u32) -> Option<Target> {
    if g.chips_y + chip_h() > g.bottom {
        return None;
    }
    let entry = state.entries.get(state.cursor)?;
    let names = state.tags.tags_for(entry.full_path.as_str());
    let slots = tag_slots(&names, g.x, g.chips_y, g.w);
    for (i, slot) in slots.iter().enumerate() {
        let rx = remove_x(slot);
        if y >= slot.y && y < slot.y + chip_h() && x >= rx && x < rx + px(X_BOX) {
            return Some(Target::Untag(i));
        }
    }
    let (ax, ay, aw) = add_box(&slots, g.x, g.chips_y, g.w);
    if y >= ay && y < ay + chip_h() && x >= ax && x < ax + aw {
        return Some(Target::AddTag);
    }
    None
}
