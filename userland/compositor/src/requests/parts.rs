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

//! A parts block: the changed box of a present or a layer's content told
//! apart as 1 to MAX_PARTS boxes, each wholly inside that box, so two rows
//! changed far apart are repainted alone and not all the rows between them.
//! The request's own box still bounds every part, so whatever reads only it
//! repaints all they name.

use super::area::Area;
use super::read::words;
use crate::protocol::{MAX_PARTS, PARTS_LEN};

/// The boxes, the first `count` of `boxes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parts {
    count: usize,
    boxes: [Area; MAX_PARTS],
}

impl Parts {
    /// The one box `a`: what a request with no parts block repaints.
    pub fn one(a: Area) -> Parts {
        let none = Area { x: 0, y: 0, width: 0, height: 0 };
        let mut boxes = [none; MAX_PARTS];
        boxes[0] = a;
        Parts { count: 1, boxes }
    }

    pub fn iter(&self) -> impl Iterator<Item = Area> + '_ {
        self.boxes[..self.count].iter().copied()
    }
}

/// The parts `block` names inside `bound`. None for a block of another
/// length, a count of 0 or over MAX_PARTS, an empty box, a box not wholly
/// inside `bound`, or a word past the count not zero.
pub fn parts(block: &[u8], bound: Area) -> Option<Parts> {
    let w = words::<{ PARTS_LEN / 4 }, PARTS_LEN>(block)?;
    let count = usize::try_from(w[0]).ok()?;
    if count == 0 || count > MAX_PARTS {
        return None;
    }
    let mut p = Parts::one(bound);
    for (i, b) in w[1..].chunks_exact(4).enumerate() {
        let a = Area { x: b[0], y: b[1], width: b[2], height: b[3] };
        if i >= count {
            if b.iter().any(|&v| v != 0) {
                return None;
            }
            continue;
        }
        if !inside(a, bound) {
            return None;
        }
        p.boxes[i] = a;
    }
    p.count = count;
    Some(p)
}

/// `a` is not empty and lies wholly in `bound`.
fn inside(a: Area, bound: Area) -> bool {
    let end = |at: u32, len: u32| at.checked_add(len);
    let (Some(r), Some(b)) = (end(a.x, a.width), end(a.y, a.height)) else { return false };
    let (Some(br), Some(bb)) = (end(bound.x, bound.width), end(bound.y, bound.height)) else {
        return false;
    };
    a.width != 0 && a.height != 0 && a.x >= bound.x && a.y >= bound.y && r <= br && b <= bb
}
