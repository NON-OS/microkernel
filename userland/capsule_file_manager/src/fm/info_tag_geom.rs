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

use super::chip::{chip_gap, chip_h, chip_w};
use super::layout::px;

// The remove affordance's box and the gap holding it off the pill. Both are
// counted into the slot, so the pill and box the painter draws and the box the
// hit-test reads come out of the same width.
pub const X_BOX: u32 = 18;
pub const X_GAP: u32 = 4;

/// One laid-out tag chip: the pill, and the box after it that removes the tag.
pub struct TagSlot<'a> {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub name: &'a str,
}

/// The slot's full width: the pill measured off its label, plus the remove box
/// that follows it.
pub fn tag_w(name: &str) -> u32 {
    chip_w(name) + px(X_GAP) + px(X_BOX)
}

/// Left edge of a chip's remove box, the gap past the end of its pill.
pub fn remove_x(slot: &TagSlot) -> u32 {
    slot.x + slot.w.saturating_sub(px(X_BOX))
}

/// The chips wrapped into `w`, in one pass the painter draws from and the
/// hit-test resolves against.
pub fn tag_slots<'a>(names: &[&'a str], x: u32, top: u32, w: u32) -> Vec<TagSlot<'a>> {
    let mut out = Vec::new();
    let (mut cx, mut y) = (x, top);
    for name in names {
        let cw = tag_w(name);
        if cx > x && cx + cw > x + w {
            cx = x;
            y += chip_h() + chip_gap();
        }
        out.push(TagSlot { x: cx, y, w: cw, name });
        cx += cw + chip_gap();
    }
    out
}
