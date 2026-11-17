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

//! The ribbon's toggle and paragraph cells as squares: the icon each shows,
//! and whether it is in force and backed by the model. The painter and the hover
//! both read the cells the painter recorded, so a square lights where it was
//! drawn.

use nonos_toolkit::widgets::Rect;

use super::items::{RibbonItem, ICONS, ICON_ALIGN, ICON_LIST, ICON_LIVE, TOGGLES, TOGGLE_LIVE};
use super::metrics::{band_top, cell_h, cell_top, RibbonCell};
use super::snapshot::RibbonState;
use super::square::Square;
use crate::editor::layout::ribbon_h;

/// The square cell `c` draws as, its column the band's full height, or None
/// for a pill.
pub(super) fn square(c: &RibbonCell, st: &RibbonState) -> Option<Square> {
    let band = Rect { x: c.x0, y: band_top(), w: c.x1 - c.x0, h: ribbon_h() };
    let body = Rect { y: cell_top(), h: cell_h(), ..band };
    let (icon, on, live) = match c.item {
        RibbonItem::Pill(_) => return None,
        RibbonItem::Toggle(t) => {
            let live = TOGGLE_LIVE.get(t).copied().unwrap_or(false);
            (TOGGLES[t], live && st.flags[t], live)
        }
        RibbonItem::Icon(k) => {
            let align = ICON_ALIGN.get(k).copied().flatten();
            let listed = ICON_LIST.get(k).copied().flatten();
            let on = align == Some(st.align) || (listed.is_some() && listed == st.list);
            (ICONS[k], on, ICON_LIVE.get(k).copied().unwrap_or(false))
        }
    };
    Some(Square { band, body, icon, on, live })
}
