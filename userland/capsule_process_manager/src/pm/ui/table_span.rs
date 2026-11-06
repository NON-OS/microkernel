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

//! Where the shared table puts its rows and columns, measured from its top
//! left, for what the app draws over a cell and for the visible slice.

use alloc::vec::Vec;

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::rect::share;

use super::super::metrics::ROW_H;

/// How tall the header and each row are, in drawing pixels.
pub fn row_h() -> u32 {
    scale().px(ROW_H)
}

/// The header and the hairline under it, where the first row starts.
pub fn head_h() -> u32 {
    row_h().saturating_add(scale().hair())
}

pub fn row_y(index: usize) -> u32 {
    head_h() + index as u32 * row_h()
}

pub fn visible_rows(table_h: u32) -> usize {
    (table_h.saturating_sub(head_h()) / row_h().max(1)).max(1) as usize
}

/// Each column's left edge and width across `w`, as the shared table shares
/// it: `fixed` holds each column's logical width, 0 for one share of what the
/// fixed columns leave. They shrink in proportion when `w` is too narrow, and
/// the last takes any pixel left over.
pub fn spans(fixed: &[u32], w: u32) -> Vec<(u32, u32)> {
    let s = scale();
    let want = |f: &u32| (s.px(*f), u32::from(*f == 0));
    let add = |a: (u32, u32), b: (u32, u32)| (a.0.saturating_add(b.0), a.1.saturating_add(b.1));
    let (all, shares) = fixed.iter().map(want).fold((0, 0), add);
    let room = w.saturating_sub(all);
    let (mut seen, mut x, mut out) = ((0, 0), 0u32, Vec::with_capacity(fixed.len()));
    for f in fixed {
        let next = add(seen, want(f));
        let cut = |n: (u32, u32), len: u32, of: u32| share(n.1, len, of) - share(n.0, len, of);
        let a = if w >= all { want(f).0 } else { cut((seen.0, next.0), w, all) };
        let b = cut((seen.1, next.1), room, shares);
        out.push((x, a.saturating_add(b)));
        (x, seen) = (x.saturating_add(a).saturating_add(b), next);
    }
    if let Some(last) = out.last_mut() {
        last.1 = last.1.saturating_add(w.saturating_sub(x));
    }
    out
}

/// The part of a span its text keeps to: `SPACE.md` in from each edge, or
/// half its width when narrower.
pub fn ink((x, w): (u32, u32)) -> (u32, u32) {
    let pad = scale().px(u32::from(SPACE.md)).min(w / 2);
    (x.saturating_add(pad), w - pad * 2)
}
