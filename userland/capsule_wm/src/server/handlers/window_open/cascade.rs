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

use crate::geometry::Rect;

use super::constants::{dock_for, menubar_for, PLACEMENT_GAP, PLACEMENT_STEP};

// Cascade resets after this many windows so they never march off-screen.
const CASCADE_WRAP: u32 = 5;

/// Where a normal window of the `requested` size opens on a `display_w` by
/// `display_h` screen with `open` normal windows already showing. The work
/// area is the screen between the menubar and the dock's band: the first
/// window sits in its middle, and each later one of a run of five steps down
/// and right from it, never past the work area's right or bottom edge.
pub(crate) fn cascade(display_w: u32, display_h: u32, open: u32, requested: Rect) -> Rect {
    let step = PLACEMENT_STEP + PLACEMENT_GAP;
    let slot = open % CASCADE_WRAP;
    let max_x = display_w.saturating_sub(requested.width);
    let bar = menubar_for(display_w, display_h);
    let dock = dock_for(display_w, display_h);
    // Clear of the dock where the window fits above it; a taller window still
    // keeps its title bar below the menubar, and only then reaches the dock.
    let above_dock = display_h.saturating_sub(dock + requested.height);
    let max_y =
        if above_dock >= bar { above_dock } else { display_h.saturating_sub(requested.height) };

    // The centred origin, between the menubar and the dock, for this size.
    let centre_x = max_x / 2;
    let work = display_h.saturating_sub(bar + dock);
    let centre_y = bar + work.saturating_sub(requested.height) / 2;

    // The first window of a run sits exactly in the middle, because that is
    // the case that happens most and the one a person notices. Later windows
    // step down and right from it so their titlebars stay reachable.
    //
    // An earlier version shifted the whole cascade back by half its run to
    // centre the group. That is the wrong thing to optimise: it left a lone
    // window a hundred and twenty-eight pixels left of centre, which on a wide
    // window with little room to move reads as "stuck near the edge".
    let x = centre_x.saturating_add(slot * step).min(max_x);
    // The bar's floor is applied last: a window too tall for the screen below
    // the bar still opens with its title bar under it, and the compositor
    // clips the foot that runs off the screen. Applied first, the cap pulled
    // such a window up under the bar, title bar and close button with it.
    let y = centre_y.saturating_add(slot * step).min(max_y).max(bar);

    Rect { x, y, width: requested.width, height: requested.height }
}
