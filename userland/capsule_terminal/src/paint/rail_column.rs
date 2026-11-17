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

//! The rail as it was drawn: its window rect, its clamped scroll and the
//! length of each list. The painter, the hover and the click all place the
//! lists from one of these, so a click lands on the row drawn under it.

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Pointer, Rect, State};

use super::rail_nav::nav;
use super::rail_spot::{spot, Spot};

#[derive(Clone, Copy, Debug)]
pub struct Column {
    pub rect: Rect,
    pub off: u32,
    pub sessions: u32,
    pub projects: u32,
}

impl Column {
    /// The window point in the column's content pixels, when it lies on the rail.
    pub fn local(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        let (rx, ry) = (self.rect.x as i32, self.rect.y as i32);
        contains(self.rect, x, y).then(|| (x - rx, y - ry + self.off as i32))
    }

    /// The control under the window point.
    pub fn spot(&self, x: i32, y: i32) -> Option<Spot> {
        let (x, y) = self.local(x, y)?;
        spot(&nav(self.rect.w, self.sessions, self.projects), self.sessions, self.projects, x, y)
    }

    /// The hover and press the control drawn in content box `b` shows.
    pub fn look(&self, ptr: &Pointer, b: Rect) -> State {
        ptr.state(|x, y| self.local(x, y).is_some_and(|(x, y)| contains(b, x, y)))
    }

    /// Whether any of content box `b` lies in the rail's view.
    pub fn shows(&self, b: Rect) -> bool {
        b.y < self.off.saturating_add(self.rect.h) && b.y.saturating_add(b.h) > self.off
    }

    /// The control the pointer is over, if any.
    pub fn lit(&self, ptr: &Pointer) -> Option<Spot> {
        ptr.position().and_then(|(x, y)| self.spot(x, y))
    }
}
