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

//! The Home screen below the activity strip: the rail, the pane head and,
//! when the window is wide enough, the document card. Every control is laid
//! out afresh in `c` on each paint.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

use super::doc_ref::DocRef;
use super::metrics::{pane_x, PANE_PAD};
use super::pane::paint_pane_head;
use super::pick::HomePick;
use super::rail::paint_rail;
use super::recent::paint_recent;

const MIN_COLS_W: u32 = 200;

pub(super) fn paint_screen(
    fb: &mut PaintBuffer,
    nav: usize,
    list: &[DocRef],
    c: &mut Controls<HomePick>,
) {
    c.clear();
    if fb.height == 0 || fb.width <= pane_x() + PANE_PAD * 2 {
        return;
    }
    paint_rail(fb, nav, c);
    paint_pane_head(fb);
    if fb.width >= pane_x() + PANE_PAD * 2 + MIN_COLS_W {
        paint_recent(fb, nav, list, c);
    }
}
