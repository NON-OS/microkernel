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

//! Whether a frame drawn into a buffer may draw its damage alone. The
//! compositor composes only what a present damages, so the buffer a layer
//! shows must hold the whole picture: a buffer taken again is out of date by
//! what the other presented since (`Stale`). An app that names a damage rect
//! paints that rect alone (`App::take_damage`), so it is enough while the
//! rect holds all that is out of date; otherwise the whole window is drawn.
//! Judged against the content area, a browser typing into its home page
//! repainted the search box alone over a buffer still holding its first,
//! unfinished frame, and showed that frame with no toolbar and no tiles.

use crate::swap::Stale;

/// `area` is the box the frame draws, (x, y, width, height) in the buffer's
/// pixels.
pub fn whole_needed(stale: Stale, area: (u32, u32, u32, u32)) -> bool {
    let (cx, cy, cw, ch) = area;
    match stale {
        Stale::None => false,
        Stale::Whole => true,
        Stale::Box(d) => {
            let inside_x = d.x >= cx && d.x.saturating_add(d.width) <= cx.saturating_add(cw);
            let inside_y = d.y >= cy && d.y.saturating_add(d.height) <= cy.saturating_add(ch);
            !(inside_x && inside_y)
        }
    }
}
