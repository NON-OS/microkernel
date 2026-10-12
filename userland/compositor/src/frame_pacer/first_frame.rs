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

//! The first frame's hold as the frame pacer applies it: every present goes
//! through `tick`, so holding here keeps the firmware's picture on screen
//! whichever path (a frame, the band, the cursor) would have shown first.

use crate::display::first_frame_rule::held;
use crate::state::Context;

/// True while nothing may reach the display yet; lets go for good once
/// the rule says so, so a scene that empties later shows as empty.
pub fn holding(ctx: &mut Context) -> bool {
    let Some(since) = ctx.first_frame_since else {
        return false;
    };
    if held(Some(since), nonos_libc::mk_uptime_ms(), ctx.scene.layers().count()) {
        return true;
    }
    ctx.first_frame_since = None;
    false
}
