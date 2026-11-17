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

//! The open pill's dropdown, painted with the overlays so it sits above the
//! pane. Every row here is backed by the document model, so none draw dimmed.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Pointer, Rect};

use super::items::pill_labels;
use super::metrics::RibbonCell;
use super::panel::{panel_at, picked};
use super::snapshot::RibbonState;
use crate::editor::menu_view::{draw, lit, ticked};

pub(in crate::editor) fn paint_ribbon_drop(
    fb: &mut PaintBuffer,
    cells: &[RibbonCell],
    pill: usize,
    st: &RibbonState,
    bounds: Rect,
    pointer: &Pointer,
) {
    let labels = pill_labels(pill);
    let rows = ticked(&labels, picked(st, pill));
    draw(fb, &rows, panel_at(cells, pill, &rows, bounds), pointer);
}

/// The row of the open pill's dropdown the pointer lights.
pub(in crate::editor) fn ribbon_drop_lit(
    cells: &[RibbonCell],
    pill: usize,
    st: &RibbonState,
    bounds: Rect,
    pointer: &Pointer,
) -> Option<usize> {
    let labels = pill_labels(pill);
    let rows = ticked(&labels, picked(st, pill));
    lit(&rows, panel_at(cells, pill, &rows, bounds), pointer)
}
