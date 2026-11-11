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

//! Geometry shared by the shell's painter and its event router, so a click is
//! tested against the exact rectangle the corresponding panel was drawn in.

use super::layout::{activity_w, ribbon_h, sidebar_w, tabbar_h, titlebar_h};

// Left edge of the code pane: past the activity bar, plus the sidebar when open.
pub(super) fn pane_x(sidebar_open: bool) -> u32 {
    activity_w() + if sidebar_open { sidebar_w() } else { 0 }
}

// Top edge of the code pane: below the menu bar, the tab strip, and the ribbon.
// The sidebar tree hangs off this too, and the ribbon band spans the whole
// window right of the activity bar, so both stay flush with no seam.
/// Top of the editing pane.
///
/// The formatting ribbon belongs to the document view. In the code view there
/// is nothing for it to format, and forty pixels of bold, italic and paragraph
/// alignment above a source file is chrome from a different program. So the
/// pane starts higher when the ribbon is not drawn.
pub(super) fn pane_y(ribbon: bool) -> u32 {
    titlebar_h() + tabbar_h() + if ribbon { ribbon_h() } else { 0 }
}

// The code pane rectangle for a given window size and sidebar state.
pub(super) fn pane_rect(
    width: u32,
    height: u32,
    sidebar_open: bool,
    ribbon: bool,
) -> (u32, u32, u32, u32) {
    let x = pane_x(sidebar_open);
    let y = pane_y(ribbon);
    let w = width.saturating_sub(x);
    let h = height.saturating_sub(y + super::layout::footer_h());
    (x, y, w, h)
}
