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

use crate::browser::css::Size;

use super::super::abs_out_of_flow::out_of_flow;
use super::super::box_kind::inner;
use super::super::replaced_size::replaced_size;
use super::super::tree::{BoxKind, BoxNode};
use super::intrinsic::CAP;
use super::intrinsic_box::{flex_row, grid, stack};
use super::intrinsic_inline::inline_run;

/* The (min, max) content widths of `n` inside its padding and border, by
 * the formatting context it runs: an image's own width, an inline run's
 * words, a flex row's items side by side, a grid's columns, or stacked
 * blocks (a flex column too). */
pub(in super::super) fn content(n: &BoxNode, depth: u32) -> (i32, i32) {
    let s = &n.style;
    let in_flow = || n.children.iter().filter(|c| !out_of_flow(&c.style));
    match inner(n) {
        BoxKind::Image { .. } => image(n),
        BoxKind::Text(_) => inline_run(core::slice::from_ref(n), depth),
        BoxKind::Flex if !s.flex_col => flex_row(n, depth),
        BoxKind::Flex => stack(n, depth),
        _ if s.is_grid => grid(n, depth),
        _ if in_flow().all(|c| !c.kind.block_level()) => inline_run(&n.children, depth),
        _ => stack(n, depth),
    }
}

/* A width relative to the container cannot size an image here: it offers
 * its own width (attribute, else natural, else nothing yet) and, like one
 * with a relative max-width, may shrink to nothing. */
fn image(n: &BoxNode) -> (i32, i32) {
    let s = &n.style;
    let own = || n.aux.attr[0].or(n.aux.natural.map(|v| v.0)).map_or(0, |w| w as i32);
    let relative = s.width != Size::Auto;
    let w = if relative { own() } else { replaced_size(n, CAP, None).0 };
    (if relative || matches!(s.max_width, Size::Pct(_)) { 0 } else { w }, w)
}
