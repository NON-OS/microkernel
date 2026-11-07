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

//! The edges a resize from the left or the top holds where they were. The
//! configure tells the guest a size and the corner it was worked out for;
//! a guest that draws another size (a terminal keeping whole character
//! cells, a minimum of its own) had its buffer placed at that corner, so the
//! right or the bottom edge, which the resize never moved, moved with it.
//! Held, the window's corner is worked out from the size the guest drew.
//! Pure, so the host proofs hold it.

use super::mode::Rect;
use super::place::origin;
use super::resize::Resize;
use super::state::Fit;

/// Where a resize holds the window's far edges on screen, frame and all.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Held {
    /// The right edge, when the left one was moved.
    pub right: Option<u32>,
    /// The bottom edge, when the top one was moved.
    pub bottom: Option<u32>,
    /// The menubar's foot, which no corner goes above.
    pub top: u32,
}

impl Held {
    /// What resize `r` holds once it asked for `rect` (corner and content
    /// size) in a frame `frame` wide and high, under a menubar `top` high.
    pub fn of(r: &Resize, rect: Rect, frame: (u32, u32), top: u32) -> Held {
        let end = |at: u32, len: u32, f: u32| at.saturating_add(len).saturating_add(f);
        Held {
            right: r.sides.left.then(|| end(rect.0, rect.2, frame.0)),
            bottom: r.sides.top.then(|| end(rect.1, rect.3, frame.1)),
            top,
        }
    }

    /// The corner a window `w` by `h`, frame and all, goes to from `at`,
    /// the corner the configure named: its held edges where they were.
    pub fn corner(&self, (x, y): (u32, u32), (w, h): (u32, u32)) -> (u32, u32) {
        let x = self.right.map_or(x, |r| r.saturating_sub(w));
        let y = self.bottom.map_or(y, |b| b.saturating_sub(h).max(self.top));
        (x, y)
    }
}

impl Fit {
    /// The corner a buffer `w` by `h`, frame and all, is shown at: the acked
    /// configure's (origin), its held edges kept where a resize held them.
    pub fn corner(&self, at: Option<(u32, u32)>, w: u32, h: u32) -> (u32, u32) {
        let corner = origin(self.due(), at, w, h);
        match (self.due().and_then(|c| c.rect), self.acked_held) {
            (Some(_), Some(held)) => held.corner(corner, (w, h)),
            _ => corner,
        }
    }
}
