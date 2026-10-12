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

//! One canvas across several outputs (unit 10, the shell's "extend"): each
//! output takes a slice of the canvas side by side, output 0 at the left
//! with the canvas it would have alone, every further output as many canvas
//! pixels as its screen holds at the same scale; the canvas is as tall as
//! the tallest slice. Pure, so the host proofs pin it.

use crate::state::damage::Rect;

/// The wire server's outputs (`nonos_gfx_serve::MAX_OUTPUTS`).
pub const MAX_SLICES: usize = 4;

/// An output's part of the canvas: from `x`, `w` wide, `h` tall.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slice {
    pub x: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Plan {
    pub size: (u32, u32),
    pub slices: [Slice; MAX_SLICES],
    pub n: usize,
}

/// The canvas for output 0's own canvas `first` and further screens
/// `heads` at `scale`; None when a slice would be empty or too wide.
pub fn plan(first: (u32, u32), heads: &[(u32, u32)], scale: u32) -> Option<Plan> {
    let mut p = Plan::default();
    let mut add = |w: u32, h: u32| -> Option<()> {
        let slot = p.slices.get_mut(p.n)?;
        (w > 0 && h > 0).then_some(())?;
        *slot = Slice { x: p.size.0, w, h };
        p.size = (p.size.0.checked_add(w)?, p.size.1.max(h));
        p.n += 1;
        Some(())
    };
    add(first.0, first.1)?;
    for &(w, h) in heads {
        add(w / scale.max(1), h / scale.max(1))?;
    }
    // Four outputs of 4096 at most: a wider canvas is not one anything shows.
    (p.size.0 <= 16384).then_some(p)
}

/// The part of canvas rectangle `r` in slice `s`, in the slice's own
/// coordinates; None when none of it is.
pub fn local(r: Rect, s: Slice) -> Option<Rect> {
    let x0 = r.x.max(s.x);
    let x1 = r.x.saturating_add(r.width).min(s.x + s.w);
    let y1 = r.y.saturating_add(r.height).min(s.h);
    (x0 < x1 && r.y < y1).then(|| Rect { x: x0 - s.x, y: r.y, width: x1 - x0, height: y1 - r.y })
}
