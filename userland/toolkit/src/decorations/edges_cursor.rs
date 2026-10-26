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
//! The frame's measures at one to one (HANDOFF/ui/identity.md, Window
//! The pointer's shape over a window's edges: what a press there would do.

use super::edges::Edges;

/// The pointer's shapes over the frame, as the compositor numbers them
/// (compositor frame_pacer/cursor_shapes.rs).
pub const ARROW_CURSOR: u8 = 0;
pub const EW_CURSOR: u8 = 1;
pub const NS_CURSOR: u8 = 2;
pub const NWSE_CURSOR: u8 = 3;
pub const NESW_CURSOR: u8 = 4;

impl Edges {
    /// The pointer's shape over these edges: a corner's diagonal, an edge's
    /// straight arrow, or the arrow over none.
    pub fn cursor(self) -> u8 {
        match (self.left, self.right, self.top, self.bottom) {
            (true, _, true, _) | (_, true, _, true) => NWSE_CURSOR,
            (_, true, true, _) | (true, _, _, true) => NESW_CURSOR,
            (true, ..) | (_, true, ..) => EW_CURSOR,
            (.., true, _) | (.., true) => NS_CURSOR,
            _ => ARROW_CURSOR,
        }
    }
}
