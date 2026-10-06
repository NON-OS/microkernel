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

use alloc::vec::Vec;

use super::super::ctx::{Ctx, Pin};
use super::super::display_list::{DisplayList, Fragment};
use super::super::layout_box::layout_box;
use super::cell::{settle, Open};
use super::grid::Grid;

/* Where the columns sit: each one's left edge and width, the gap
 * between cells, the x and width a row box spans, and the rows' top. */
pub(super) struct Geo {
    pub xs: Vec<i32>,
    pub ws: Vec<i32>,
    pub sp: i32,
    pub row: (i32, i32),
    pub y: i32,
}

/* Lay the rows of `g`, each cell pinned to the width of the
 * columns it spans. A row is as tall as its tallest cell and its own
 * height; a cell spanning rows makes its last row taller when the rows
 * fall short of it. Each cell's box then fills its rows and its content
 * aligns vertically in it. Returns the y below the last gap. */
pub(super) fn place(g: &Grid, geo: &Geo, frags: &mut DisplayList, depth: u32, ctx: Ctx) -> i32 {
    let mut cy = geo.y + geo.sp;
    let mut open: Vec<Open> = Vec::new();
    let mut k = 0;
    for (r, row) in g.rows.iter().enumerate() {
        let r = r as u32;
        let slot = row.map(|n| {
            frags.push(Fragment::of_box(n, [geo.row.0, cy, geo.row.1, 0], &ctx));
            frags.len() - 1
        });
        let mut h = row.and_then(|n| n.style.height.definite_px()).unwrap_or(0).max(0);
        while let Some(s) = g.slots.get(k).filter(|s| s.row == r) {
            k += 1;
            let (c, n) = (s.col as usize, s.cs as usize);
            let w = geo.ws[c..c + n].iter().sum::<i32>() + geo.sp * (n as i32 - 1);
            let start = frags.len();
            let pin = Ctx { pin: Some(Pin { w, h: None }), ..ctx };
            let ch = layout_box(s.cell, geo.xs[c], cy, w, frags, depth + 1, pin);
            let (id, valign) = (s.cell.dom_id, s.cell.style.table.slack_halves());
            let last = r + s.rs - 1;
            open.push(Open { id, valign, top: cy, h: ch, frags: (start, frags.len()), last });
        }
        for p in open.iter().filter(|p| p.last == r) {
            h = h.max(p.top + p.h - cy);
        }
        let bottom = cy + h;
        open.retain(|p| p.last != r || !settle(p, bottom - p.top, frags, ctx.clip));
        if let Some(f) = slot.and_then(|i| frags.get_mut(i)) {
            f.h = h;
        }
        cy = bottom + geo.sp;
    }
    cy
}
