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

use crate::browser::css::{Clear, Float};

use super::super::abs_out_of_flow::out_of_flow;
use super::super::ctx::Ctx;
use super::super::display_list::DisplayList;
use super::super::float_ctx::FloatCtx;
use super::super::geom::margins::{collapse, margins};
use super::super::layout_box::layout_box;
use super::super::tree::BoxNode;
use super::layout_float::layout_float;

/// Stack block-level `children` in a content box at `r` ([x, y, w]) with
/// collapsed vertical margins, negative ones pulling boxes together or past
/// each other, floats to the sides. An out-of-flow child takes no space:
/// flow records where it would have gone, for its positioned ancestor.
/// Returns the height used, floats included.
pub(crate) fn block_flow(
    children: &[BoxNode],
    r: [i32; 3],
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let [x, y, w] = r;
    let mut floats = FloatCtx::new(x, w);
    let mut cy = y;
    let mut prev_mb: Option<i32> = None;
    for child in children {
        let cs = &child.style;
        let [mt, mr, mb, ml] = margins(cs, w);
        if out_of_flow(cs) {
            let at = floats.band(cy).0;
            child.aux.flow_at.set(Some((at, cy + prev_mb.unwrap_or(0), ctx)));
            continue;
        }
        if !child.kind.block_level() {
            continue;
        }
        if cs.float != Float::None {
            layout_float(child, &mut floats, cy, w, frags, depth, ctx);
            continue;
        }
        /* A cleared box starts below the floats it clears. */
        if cs.clear != Clear::None {
            cy = cy.max(floats.clear_row(cs.clear, cy));
        }
        cy += prev_mb.map_or(mt, |prev| collapse(prev, mt));
        let (band_x, band_w) = floats.band(cy);
        let avail = (band_w - ml - mr).max(0);
        cy += layout_box(child, band_x + ml, cy, avail, frags, depth + 1, ctx);
        prev_mb = Some(mb);
    }
    cy += prev_mb.unwrap_or(0);
    /* The container grows to contain floats reaching past its content. */
    (cy.max(floats.max_bottom()) - y).max(0)
}
