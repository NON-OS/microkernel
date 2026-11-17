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

//! Hit-testing against the cells the painter recorded, so a click always lands
//! on the control that was drawn under the pointer.

use nonos_toolkit::widgets::Rect;

use super::items::{pill_labels, RibbonItem};
use super::metrics::{band_top, RibbonCell};
use super::panel::{panel_at, picked};
use super::snapshot::RibbonState;
use crate::editor::layout::ribbon_h;
use crate::editor::menu_view::{hit, ticked};

pub(in crate::editor) enum RibbonHit {
    Cell(RibbonItem),
    Row(usize),
    Outside,
}

pub(in crate::editor) fn ribbon_hit(
    cells: &[RibbonCell],
    open: Option<usize>,
    st: &RibbonState,
    bounds: Rect,
    x: i32,
    y: i32,
) -> RibbonHit {
    if let Some(p) = open {
        let labels = pill_labels(p);
        let rows = ticked(&labels, picked(st, p));
        if let Some(r) = hit(&rows, panel_at(cells, p, &rows, bounds), x, y) {
            return RibbonHit::Row(r);
        }
    }
    if x < 0 || y < 0 {
        return RibbonHit::Outside;
    }
    let (x, y) = (x as u32, y as u32);
    if y >= band_top() && y < band_top() + ribbon_h() {
        for c in cells.iter() {
            if x >= c.x0 && x < c.x1 {
                return RibbonHit::Cell(c.item);
            }
        }
    }
    RibbonHit::Outside
}
