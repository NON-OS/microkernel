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

//! The pointer of a Mouse, Pointer or digitizer (pen, touchscreen)
//! collection: up to five buttons, X and Y, a wheel, and for a digitizer its
//! Tip Switch and In Range bits. X and Y both relative make a mouse, both
//! absolute a tablet; a touchpad's own absolute collection is not read, as
//! its position is not a place on the screen.

use super::layout::{Field, NONE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pointer {
    pub id: u8,
    pub app: u16,
    /// Button i + 1 is the bit at buttons[i].
    pub buttons: [u16; 5],
    pub tip: u16,
    pub range: u16,
    pub x: Option<Field>,
    pub y: Option<Field>,
    pub wheel: Option<Field>,
}

impl Pointer {
    pub fn new(id: u8, app: u16) -> Self {
        let (buttons, tip, range) = ([NONE; 5], NONE, NONE);
        Self { id, app, buttons, tip, range, x: None, y: None, wheel: None }
    }

    /// X and Y both there and of one kind; an absolute pair needs a range
    /// to scale onto the screen.
    pub fn usable(&self) -> bool {
        let (Some(x), Some(y)) = (self.x, self.y) else { return false };
        x.relative == y.relative && (x.relative || (x.max > x.min && y.max > y.min))
    }
}
