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

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::css::Size;

use super::grid::Grid;
use super::measure::intrinsic;
use super::share::widen;

/* One column: its min- and max-content border-box widths, whether a cell
 * fixed its width, and the largest percentage width a cell asked for. */
#[derive(Clone, Copy, Default)]
pub(in super::super) struct Col {
    pub min: i32,
    pub max: i32,
    pub fixed: bool,
    pub pct: u16,
}

/* The columns of grid `g` (CSS 2.1 17.5.2.2): single-column cells set
 * each column's widths, a fixed cell width holding it at that width;
 * then each spanning cell, narrowest span first, widens the columns it
 * covers by what it needs beyond them and the `sp` gaps between them. */
pub(in super::super) fn columns(g: &Grid, sp: i32, depth: u32) -> Vec<Col> {
    let mut cols = vec![Col::default(); g.ncols];
    let mut spans: Vec<(u32, i32, i32, usize, usize)> = Vec::new();
    for s in &g.slots {
        let st = &s.cell.style;
        let (mut mn, mut mx) = intrinsic(s.cell, depth);
        let edges = (st.pad_left + st.pad_right + st.border_left + st.border_right) as i32;
        let fixed = st.width.definite_px().map(|w| w + if st.border_box { 0 } else { edges });
        if let Some(w) = fixed {
            (mn, mx) = (mn.max(w), mn.max(w));
        }
        let (c, n) = (s.col as usize, s.cs as usize);
        if n > 1 {
            spans.push((s.cs, mn, mx, c, n));
            continue;
        }
        let col = &mut cols[c];
        (col.min, col.max) = (col.min.max(mn), col.max.max(mx));
        col.fixed |= fixed.is_some();
        if let Size::Pct(p) = st.width {
            col.pct = col.pct.max(p);
        }
    }
    spans.sort_by_key(|s| s.0);
    for (_, mn, mx, c, n) in spans {
        let gaps = sp * (n as i32 - 1);
        let span = &mut cols[c..c + n];
        widen(span, mn - gaps, |k| &mut k.min);
        widen(span, mx - gaps, |k| &mut k.max);
        span.iter_mut().for_each(|k| k.max = k.max.max(k.min));
    }
    cols
}
