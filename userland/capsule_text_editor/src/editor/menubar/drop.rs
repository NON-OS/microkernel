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

//! The dropdown panel for the open title, painted with the overlays so it sits
//! above the pane and the status bar.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Pointer, Rect};

use super::items::items;
use super::metrics::{panel_at, TitleSpan};
use crate::editor::menu_view::{draw, lit};

pub(in crate::editor) fn paint_dropdown(
    fb: &mut PaintBuffer,
    spans: &[TitleSpan],
    open: usize,
    bounds: Rect,
    pointer: &Pointer,
) {
    let rows = items(open);
    draw(fb, &rows, panel_at(spans, open, &rows, bounds), pointer);
}

/// The row of the open title's drop-down the pointer lights.
pub(in crate::editor) fn dropdown_lit(
    spans: &[TitleSpan],
    open: usize,
    bounds: Rect,
    pointer: &Pointer,
) -> Option<usize> {
    let rows = items(open);
    lit(&rows, panel_at(spans, open, &rows, bounds), pointer)
}
