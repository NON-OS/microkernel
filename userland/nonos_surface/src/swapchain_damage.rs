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

//! What part of a buffer is out of date when it is taken again: the union
//! of the damage every other buffer presented since it was last shown
//! (what EGL calls buffer age, kept as one box).

/// A damaged rectangle in surface pixels.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Damage {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stale {
    /// Up to date: nothing to redraw but the new frame's own damage.
    None,
    /// This box is out of date.
    Box(Damage),
    /// Never drawn: all of it.
    Whole,
}

impl Stale {
    pub fn grow(self, d: Damage) -> Stale {
        if d.width == 0 || d.height == 0 {
            return self;
        }
        match self {
            Stale::Whole => Stale::Whole,
            Stale::None => Stale::Box(d),
            Stale::Box(b) => {
                let (x0, y0) = (b.x.min(d.x), b.y.min(d.y));
                let x1 = b.x.saturating_add(b.width).max(d.x.saturating_add(d.width));
                let y1 = b.y.saturating_add(b.height).max(d.y.saturating_add(d.height));
                Stale::Box(Damage { x: x0, y: y0, width: x1 - x0, height: y1 - y0 })
            }
        }
    }
}
