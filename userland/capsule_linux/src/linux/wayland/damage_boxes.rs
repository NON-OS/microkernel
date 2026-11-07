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

//! What a client said changed since its last commit as the boxes it named,
//! kept apart, in buffer pixels: a selection near a window's top and its
//! last row stay two boxes, not the one round both that damage.rs keeps.
//! Boxes that touch and cost little more as one are joined, so rows named
//! one by one are one box; past MAX_BOXES a box is joined to the one it
//! grows least. Every box lies inside damage.rs's box of the same commit.
//! Pure, so the host proofs hold it.

use super::damage::corners;

/// A box in buffer pixels, corner to corner: x0, y0, x1, y1.
pub type Corners = (u32, u32, u32, u32);

/// At most this many boxes, as many as the compositor takes apart.
pub const MAX_BOXES: usize = 4;

/// The pixels two boxes may cost as one past their own and still be one,
/// as the compositor joins its damage (state/damage/rect.rs SLACK).
const SLACK: u64 = 64 * 64;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Boxes {
    count: usize,
    boxes: [Corners; MAX_BOXES],
}

impl Boxes {
    /// With the box at (`x`, `y`), `w` by `h`, `scale` buffer pixels to
    /// one it was named in, added as damage.rs Damage::add takes it.
    pub fn add(self, named: (i32, i32, i32, i32), scale: u32) -> Boxes {
        match corners(named, scale) {
            Some(c) => self.with(c),
            None => self,
        }
    }

    fn with(mut self, c: Corners) -> Boxes {
        let mut c = c;
        // Each pass takes one box out, so this ends within MAX_BOXES passes.
        while let Some(i) = (0..self.count).find(|&i| worth_one(self.boxes[i], c)) {
            c = union(c, self.take(i));
        }
        if self.count == MAX_BOXES {
            // Full and touching none: joined to the box it grows least,
            // which may now touch another, so it is added again.
            let j = (0..self.count).min_by_key(|&i| area(union(self.boxes[i], c))).unwrap_or(0);
            let grown = union(c, self.take(j));
            return self.with(grown);
        }
        self.boxes[self.count] = c;
        self.count += 1;
        self
    }

    /// Box `i` out; the last takes its place.
    fn take(&mut self, i: usize) -> Corners {
        let c = self.boxes[i];
        self.count -= 1;
        self.boxes[i] = self.boxes[self.count];
        c
    }

    pub fn iter(&self) -> impl Iterator<Item = Corners> + '_ {
        self.boxes[..self.count].iter().copied()
    }

    /// The boxes from (`dx`, `dy`) on, as a window cut from the buffer at
    /// that corner takes them (damage.rs Damage::moved), each cut to
    /// `bound`, the box the commit shows; those left empty dropped. So each
    /// lies inside `bound` whatever was added.
    pub fn within(&self, (dx, dy): (u32, u32), bound: Corners) -> Boxes {
        let (bx0, by0, bx1, by1) = bound;
        let mut out = Boxes::default();
        for (x0, y0, x1, y1) in self.iter() {
            let x0 = x0.saturating_sub(dx).max(bx0);
            let y0 = y0.saturating_sub(dy).max(by0);
            let (x1, y1) = (x1.saturating_sub(dx).min(bx1), y1.saturating_sub(dy).min(by1));
            if x0 < x1 && y0 < y1 {
                out.boxes[out.count] = (x0, y0, x1, y1);
                out.count += 1;
            }
        }
        out
    }

    /// The boxes inside `bound` from (`dx`, `dy`) on (`within`), when there
    /// are two or more to tell apart; one box is `bound`'s own business.
    pub fn apart(&self, cut: (u32, u32), bound: Corners) -> Option<Boxes> {
        let b = self.within(cut, bound);
        (b.count >= 2).then_some(b)
    }

    /// The boxes as x, y, width and height, the first `count` of the array.
    pub fn areas(&self) -> ([(u32, u32, u32, u32); MAX_BOXES], usize) {
        let mut out = [(0, 0, 0, 0); MAX_BOXES];
        for (o, (x0, y0, x1, y1)) in out.iter_mut().zip(self.iter()) {
            *o = (x0, y0, x1 - x0, y1 - y0);
        }
        (out, self.count)
    }

    /// How many there are.
    pub fn count(&self) -> usize {
        self.count
    }
}

fn area((x0, y0, x1, y1): Corners) -> u64 {
    u64::from(x1.saturating_sub(x0)) * u64::from(y1.saturating_sub(y0))
}

fn union(a: Corners, b: Corners) -> Corners {
    (a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3))
}

/// `a` and `b` meet or share an edge, and one box round both costs no more
/// than SLACK pixels past the two.
fn worth_one(a: Corners, b: Corners) -> bool {
    let touch = a.0 <= b.2 && b.0 <= a.2 && a.1 <= b.3 && b.1 <= a.3;
    touch && area(union(a, b)) <= area(a).saturating_add(area(b)).saturating_add(SLACK)
}
