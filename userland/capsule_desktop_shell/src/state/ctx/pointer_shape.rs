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
//! The pointer's shape over the shell's chrome. The chrome is a layer over
//! the whole display, and the compositor gives the pointer the shape of the
//! topmost layer under it that asked for one: the chrome asks for the arrow
//! while the pointer is over a pixel of its own (the menu bar, the dock, a
//! menu, a dialog) and withdraws over its transparent pixels, so a window's
//! resize arrows never show through a menu, and do show where nothing of the
//! shell's is drawn.

/// A chrome pixel at least this opaque is the shell's own.
const OWN_ALPHA: u32 = 0x80;

#[derive(Default)]
pub struct PointerShape {
    /// The pointer, as the shell last saw it move.
    pub at: Option<(u32, u32)>,
    /// What the compositor was last told: the arrow (true), withdrawn
    /// (false), or nothing yet.
    pub said: Option<bool>,
}

impl PointerShape {
    /// The chrome's ask for a pointer over this pixel of it, when it is not
    /// the one last told: Some(true) for the arrow, Some(false) to withdraw.
    pub fn change(&self, pixel: u32) -> Option<bool> {
        let over = pixel >> 24 >= OWN_ALPHA;
        (self.said != Some(over)).then_some(over)
    }
}
