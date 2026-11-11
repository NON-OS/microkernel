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

//! Where the Library and Details pages put their parts: the videos under the
//! Library's heading and the Details tabs. The painters, the click and the
//! hover all read these, so a click lands on what was drawn.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::{Rect, Tabs};

use super::rail_geom::folder_area;
use crate::ui::screen::Route;

/// The Details page's tabs; the first goes back to the player.
pub const TABS: [&str; 2] = ["Now Playing", "Media Details"];
/// How tall the band the Details tabs sit in is.
pub const STRIP_H: u32 = 42;

const HEAD: u32 = 34;

/// Whether the page shows the search field and the grid and list switch.
pub fn has_tools(route: Route) -> bool {
    matches!(route, Route::Library | Route::Files)
}

/// Where a page draws its videos, or `None` on a page with no video list.
pub fn content(route: Route, body: Rect) -> Option<Rect> {
    let head = scale().px(HEAD);
    match route {
        Route::Library => Some(Rect { y: body.y + head, h: body.h.saturating_sub(head), ..body }),
        Route::Files => Some(folder_area(body)),
        _ => None,
    }
}

/// The Library's heading, over its videos.
pub fn heading(body: Rect) -> Rect {
    Rect { h: scale().px(HEAD).min(body.h), ..body }
}

/// The Details tabs, with Media Details the page on show.
pub fn tabs() -> Tabs<'static> {
    Tabs::new(&TABS, 1)
}

/// The band across the top of the Details page the tabs sit in.
pub fn strip(body: Rect) -> Rect {
    Rect { h: scale().px(STRIP_H).min(body.h), ..body }
}
