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

//! What the frame keeps between frames: the title, the scale, the control
//! under the pointer, a drag, and the guest's own pixels.

use alloc::vec::Vec;

use nonos_toolkit::decorations::DecorationHit;

pub struct Frame {
    pub title: Vec<u8>,
    /// The display's scale in quarters, read as a window opens (4 is 1:1).
    pub quarters: u32,
    /// Shown pixels per logical pixel, in 120ths (scale_math.rs).
    pub ratio: u32,
    /// The guest draws its own frame (xdg-decoration client-side).
    pub own_frame: bool,
    pub hover: DecorationHit,
    /// Where the pointer last was on the window.
    pub pointer: (i32, i32),
    /// A held title bar: the window's corner and the pointer, on screen.
    pub drag: Option<((u32, u32), (i32, i32))>,
    /// The foot of the menubar, which a dragged window stops at.
    pub top: u32,
    /// The guest's pixels, composed with its subsurfaces and popups.
    pub client: Vec<u8>,
    /// What the frame was last painted as; None to paint it again.
    pub chrome: Option<(super::Around, DecorationHit, bool)>,
}

impl Frame {
    /// Room for `bytes` of the guest's pixels; None when the heap has none.
    pub fn client_room(&mut self, bytes: usize) -> Option<&mut [u8]> {
        if self.client.len() < bytes {
            self.client.try_reserve_exact(bytes - self.client.len()).ok()?;
            self.client.resize(bytes, 0);
        }
        self.client.get_mut(..bytes)
    }
}

impl Default for Frame {
    fn default() -> Frame {
        let (title, client) = (Vec::new(), Vec::new());
        Frame {
            title,
            quarters: 4,
            ratio: 120,
            own_frame: false,
            hover: DecorationHit::None,
            pointer: (0, 0),
            drag: None,
            top: 0,
            client,
            chrome: None,
        }
    }
}
