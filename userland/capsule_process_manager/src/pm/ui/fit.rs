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

//! How the window's width is shared out. At full size the sidebar carries its
//! labels and the inspector is docked on the right. A narrower window kept
//! both at full width and crushed the screen between them until the cards,
//! the table and the matrix drew over one another. Now the main pane keeps
//! MAIN_MIN: first the sidebar folds to its icons, then the inspector steps
//! aside, and the screen keeps the room it is laid out for.

use nonos_toolkit::design::scale::scale;

use crate::pm::state::Screen;

use super::metrics::metrics;

/// The least width the active screen is laid out for, in logical pixels.
pub const MAIN_MIN: u32 = 560;

/// The sidebar folded to its icons, in logical pixels.
pub const RAIL_W: u32 = 56;

/// Whether the sidebar shows only its icons.
pub fn rail(fb_w: u32) -> bool {
    let m = metrics();
    fb_w < m.sidebar_w + m.inspector_w + scale().px(MAIN_MIN)
}

pub fn sidebar_w(fb_w: u32) -> u32 {
    if rail(fb_w) {
        scale().px(RAIL_W)
    } else {
        metrics().sidebar_w
    }
}

/// Whether the inspector is docked on this screen at this width.
pub fn inspector(screen: Screen, fb_w: u32) -> bool {
    let s = scale();
    screen.has_inspector() && fb_w >= s.px(RAIL_W) + metrics().inspector_w + s.px(MAIN_MIN)
}
