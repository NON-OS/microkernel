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

//! A wl_region as the rectangles its client added and took away, in order,
//! and whether it covers a whole surface. Kept only so far: a region of
//! more steps is taken to cover nothing, so a surface is never called
//! opaque where it may not be. Pure, so capsule_linux_proofs holds it.

/// The steps a region keeps before it is taken to cover nothing.
pub const MAX_STEPS: usize = 16;

/// One wl_region.add (true) or wl_region.subtract (false): x, y, width
/// and height in surface pixels.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Step {
    pub add: bool,
    pub rect: (i32, i32, i32, i32),
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Steps {
    steps: [Step; MAX_STEPS],
    len: usize,
    /// More steps came than are kept.
    lost: bool,
}

impl Steps {
    /// An add or a subtract, kept while there is room.
    pub fn push(&mut self, step: Step) {
        match self.steps.get_mut(self.len) {
            Some(slot) => {
                *slot = step;
                self.len += 1;
            }
            None => self.lost = true,
        }
    }

    /// Whether the region holds every pixel of a `w` by `h` surface: an
    /// added rectangle holds it whole and no step after takes any of it
    /// away. A union of smaller rectangles that would hold it is not
    /// counted, so the answer is never yes where the region has a hole.
    pub fn covers(&self, w: u32, h: u32) -> bool {
        if self.lost || w == 0 || h == 0 {
            return false;
        }
        let mut covered = false;
        for s in &self.steps[..self.len] {
            if s.add && holds(s.rect, w, h) {
                covered = true;
            } else if !s.add && meets(s.rect, w, h) {
                covered = false;
            }
        }
        covered
    }
}

/// The rectangle holds all of (0, 0, w, h). In i64, so no sum wraps.
fn holds((x, y, rw, rh): (i32, i32, i32, i32), w: u32, h: u32) -> bool {
    let (x, y, rw, rh) = (i64::from(x), i64::from(y), i64::from(rw), i64::from(rh));
    x <= 0 && y <= 0 && x + rw >= i64::from(w) && y + rh >= i64::from(h)
}

/// The rectangle takes in some pixel of (0, 0, w, h).
fn meets((x, y, rw, rh): (i32, i32, i32, i32), w: u32, h: u32) -> bool {
    let (x, y, rw, rh) = (i64::from(x), i64::from(y), i64::from(rw), i64::from(rh));
    rw > 0 && rh > 0 && x < i64::from(w) && y < i64::from(h) && x + rw > 0 && y + rh > 0
}
