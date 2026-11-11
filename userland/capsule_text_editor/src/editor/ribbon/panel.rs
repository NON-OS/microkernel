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

//! An open pill's dropdown as the shared `Menu`, and where it opens. The
//! painter and the hit test both build it here, so a click lands on the row
//! it was drawn under.

use nonos_toolkit::widgets::{Item, Rect};

use super::items::RibbonItem;
use super::metrics::{band_top, RibbonCell};
use super::snapshot::RibbonState;
use crate::editor::layout::{activity_w, ribbon_h};
use crate::editor::menu_view::place;

/// The value pill `pill` shows for the caret.
pub(super) fn picked(st: &RibbonState, pill: usize) -> &str {
    match pill {
        0 => &st.style,
        1 => &st.font,
        _ => &st.size,
    }
}

/// Where the dropdown of `items` for pill `pill` is drawn: right under the
/// pill, kept inside `bounds`.
pub(super) fn panel_at(cells: &[RibbonCell], pill: usize, items: &[Item], bounds: Rect) -> Rect {
    let cell = cells.iter().find(|c| c.item == RibbonItem::Pill(pill));
    let (x0, x1) = cell.map_or((activity_w(), activity_w()), |c| (c.x0, c.x1));
    place(items, Rect { x: x0, y: band_top(), w: x1 - x0, h: ribbon_h() }, bounds)
}
