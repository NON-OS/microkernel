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
//! One of the shell's surfaces, kept as two. A frame is finished in the one
//! the compositor is not reading, then shown by submitting that one in its
//! place, which the compositor takes in a single step (its scene_submit keeps
//! the layer's stack and its pointer shape). The compositor so never reads a
//! frame being written: copied straight over the shown surface, row by row, a
//! composite in between showed the top of a new frame over the rest of the
//! old (booted, a half-drawn window switcher strip).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Flip {
    /// The two backings, and their compositor handles (0 before setup has
    /// registered them).
    pub va: [u64; 2],
    pub handle: [u64; 2],
    /// Which of the two the compositor reads.
    pub shown: usize,
}

impl Flip {
    /// One backing, drawn on in place, as before there were two.
    pub const fn single(va: u64) -> Self {
        Flip { va: [va, va], handle: [0, 0], shown: 0 }
    }

    /// Whether frames are finished off screen and flipped in.
    pub fn flips(&self) -> bool {
        self.handle[0] != 0 && self.handle[1] != 0 && self.va[0] != self.va[1]
    }

    /// Where the next frame is drawn: the backing not shown, when there are two.
    pub fn next_va(&self) -> u64 {
        self.va[if self.flips() { 1 - self.shown } else { self.shown }]
    }

    /// The handle the next frame is shown by.
    pub fn next_handle(&self) -> u64 {
        self.handle[1 - self.shown]
    }

    pub fn shown_va(&self) -> u64 {
        self.va[self.shown]
    }

    pub fn shown_handle(&self) -> u64 {
        self.handle[self.shown]
    }

    /// The compositor took the next frame: it is the shown one now.
    pub fn flipped(&mut self) {
        self.shown = 1 - self.shown;
    }
}
