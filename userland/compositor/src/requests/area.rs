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

//! DAMAGE_COMMIT: the rectangle a client repainted.

use super::read::words;
use crate::protocol::DAMAGE_COMMIT_REQ_LEN;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn damage_commit(body: &[u8]) -> Option<Area> {
    let [x, y, width, height] = words::<4, DAMAGE_COMMIT_REQ_LEN>(body)?;
    Some(Area { x, y, width, height })
}

impl Area {
    /// The part on a `w` by `h` canvas. Only an empty rectangle or one
    /// starting off the canvas is None; one running past an edge is cut
    /// to it, since a window manager's size can sit an edge pixel off the
    /// compositor's and a refusal there left a first paint blank.
    pub fn clip(self, w: u32, h: u32) -> Option<Area> {
        let Area { x, y, width, height } = self;
        if width == 0 || height == 0 || x >= w || y >= h {
            return None;
        }
        Some(Area { x, y, width: width.min(w - x), height: height.min(h - y) })
    }
}
