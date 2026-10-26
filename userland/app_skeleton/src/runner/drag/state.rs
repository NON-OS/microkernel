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
//! What a press on the frame holds while the pointer moves.

use nonos_toolkit::decorations::{DecorationHit, Edges, Placed};

use super::super::press_part::PressGrab;

pub(in crate::runner) struct DragState {
    pub active: bool,
    pub hover: DecorationHit,
    pub press: PressGrab,
    pub(in crate::runner) press_x: i32,
    pub(in crate::runner) press_y: i32,
    pub(in crate::runner) base_x: u32,
    pub(in crate::runner) base_y: u32,
    /// The edges a resize holds, the rect it started from, and where it is.
    pub(in crate::runner) resizing: Option<(Edges, Placed, Placed)>,
    /// The pointer's shape the frame under it wants, and the one last asked
    /// of the compositor (window_ops/tell_cursor.rs).
    pub(in crate::runner) cursor: (u8, u8),
}

impl DragState {
    pub(in crate::runner) const fn new() -> Self {
        Self {
            active: false,
            hover: DecorationHit::None,
            press: PressGrab::new(),
            press_x: 0,
            press_y: 0,
            base_x: 0,
            base_y: 0,
            resizing: None,
            cursor: (0, 0),
        }
    }
}

pub(in crate::runner) enum PointerAction {
    None,
    MoveTo(u32, u32),
    /// A drag of the right or bottom edge: the origin stays.
    ResizeTo(u32, u32),
    /// A drag that moved the left or top edge: origin and size together.
    Place(Placed),
    HoverChanged,
}
