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

//! Place a newly opened window.
//!
//! Centred, then cascaded. A window opens in the middle of the screen, which is
//! where a person is looking and where every desktop they have used puts one.
//! Successive windows step down and right from that centre so their titlebars
//! stay reachable and clicking one remains a reliable way to switch, which is
//! what the cascade was always for.
//!
//! It used to cascade from a fixed top-left corner instead, so the first window
//! of a session opened in the upper left whatever its size, and each
//! application also carried its own hand-picked origin that this then ignored.
//! Two sources of position, neither of them the middle.
//!
//! Non-normal windows (dialogs, tooltips) keep their requested position.

use crate::geometry::{clamp_to_display, Rect};
use crate::state::Context;
use crate::window::{Kind, Visibility};

use super::constants::{MENUBAR_H, PLACEMENT_GAP, PLACEMENT_STEP};

// Cascade resets after this many windows so they never march off-screen.
const CASCADE_WRAP: u32 = 5;

pub(super) fn place(ctx: &Context, kind: Kind, requested: Rect) -> Rect {
    let requested = clamp_to_display(requested, ctx.display_width, ctx.display_height);
    if kind != Kind::Normal {
        return requested;
    }
    let open = ctx
        .windows
        .windows()
        .filter(|w| w.kind == Kind::Normal && w.visibility == Visibility::Visible)
        .count() as u32;
    let step = PLACEMENT_STEP + PLACEMENT_GAP;
    let slot = open % CASCADE_WRAP;
    let max_x = ctx.display_width.saturating_sub(requested.width);
    let max_y = ctx.display_height.saturating_sub(requested.height);

    // The centred origin, below the menubar, for a window of this size.
    let centre_x = max_x / 2;
    let centre_y = MENUBAR_H + ctx.display_height.saturating_sub(MENUBAR_H + requested.height) / 2;

    // The first window of a run sits exactly in the middle, because that is
    // the case that happens most and the one a person notices. Later windows
    // step down and right from it so their titlebars stay reachable.
    //
    // An earlier version shifted the whole cascade back by half its run to
    // centre the group. That is the wrong thing to optimise: it left a lone
    // window a hundred and twenty-eight pixels left of centre, which on a wide
    // window with little room to move reads as "stuck near the edge".
    let x = centre_x.saturating_add(slot * step).min(max_x);
    let y = centre_y.saturating_add(slot * step).max(MENUBAR_H).min(max_y);

    Rect { x, y, width: requested.width, height: requested.height }
}
