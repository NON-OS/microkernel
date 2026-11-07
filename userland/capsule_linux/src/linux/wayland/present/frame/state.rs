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

use super::super::super::fit::Resize;
use super::super::super::reshape::Shape;

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
    /// A held edge: the frame's border, or one a client-drawn frame asked
    /// to resize by (toplevel/resize.rs).
    pub resize: Option<Resize>,
    /// The frame set the pointer's shape (a resize arrow over its edges),
    /// so the arrow is put back as the pointer goes onto the content.
    pub shaped: bool,
    /// The foot of the menubar, which a dragged window stops at.
    pub top: u32,
    /// The guest's pixels, composed with its subsurfaces and popups.
    pub client: Vec<u8>,
    /// The shape `client` holds pixels of, None while a write into it has
    /// not finished. A show that filled it for a new shape and then failed
    /// (no room, the window manager refusing) left the window at its old
    /// shape over pixels of the new one, and a repaint of the frame (a
    /// control lit, a title) or a partial present laid them at the old
    /// stride: every row sheared from the one above.
    pub client_shape: Option<Shape>,
    /// A buffer reused to read a commit's changed rows (damage_io.rs), kept
    /// between frames so a partial present allocates none.
    pub scratch: Vec<u8>,
    /// What the frame was last painted as; None to paint it again.
    pub chrome: Option<(super::Around, DecorationHit, bool)>,
}

impl Frame {
    /// Whether `client` holds a whole picture of `shape`.
    pub fn holds(&self, shape: Shape) -> bool {
        self.client_shape == Some(shape)
    }

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
        let (title, client, scratch) = (Vec::new(), Vec::new(), Vec::new());
        Frame {
            title,
            quarters: 4,
            ratio: 120,
            own_frame: false,
            hover: DecorationHit::None,
            pointer: (0, 0),
            drag: None,
            resize: None,
            shaped: false,
            top: 0,
            client,
            client_shape: None,
            scratch,
            chrome: None,
        }
    }
}
