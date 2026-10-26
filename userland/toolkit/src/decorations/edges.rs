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
//! Which of a window's edges a point grabs, and the rect a drag of them
//! leaves. Every edge and corner resizes: a band along the left, right and
//! bottom of the frame and its shadow, and a thinner one along the top that
//! ends where the controls begin. The sides are bands only below the title
//! bar or in a top corner, so the bar still moves the window and the
//! controls still take their presses.

use super::frame_rect::margin_at;
use super::inner_rect::titlebar_rect_at;
use super::scale::at;

/// The band inside the frame along the left, right and bottom edges.
pub const EDGE_PX: u32 = 10;
/// The band along the top: the frame's top down to the controls.
pub const TOP_EDGE_PX: u32 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Edges {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Edges {
    pub fn any(self) -> bool {
        self.left || self.right || self.top || self.bottom
    }
}

/// The edges a press at `x`, `y` in a `w` by `h` window grabs; none for a
/// maximised window, which has no edge to drag.
pub fn edges_at(w: u32, h: u32, maximized: bool, x: u32, y: u32, quarters: u32) -> Edges {
    if maximized {
        return Edges::default();
    }
    let (m, e) = (margin_at(false, quarters), at(EDGE_PX, quarters));
    let top = y < m + at(TOP_EDGE_PX, quarters);
    let bar = titlebar_rect_at(w, h, false, quarters);
    let sides = top || y >= bar.y + bar.h;
    Edges { left: sides && x < m + e, right: sides && x + e + m >= w, top, bottom: y + e + m >= h }
}
