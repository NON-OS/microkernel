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
//! The 500 ms look at a driver on the legacy ops (G-73): HEALTHCHECK, which
//! is also what keeps the driver from taking this compositor for dead. A
//! LOST answer means the driver blanked the screen while this compositor
//! was silent: the scanout is set again with the next frame, which is the
//! whole scene.

use crate::gfx_client::{self, health::LOST};
use crate::state::Context;

/// What changed, for the log; None when nothing did.
pub fn poll_legacy(ctx: &mut Context) -> Option<&'static str> {
    let request_id = ctx.issue_request_id();
    let flags = gfx_client::health::health(ctx.gfx_port, request_id).ok()?;
    if flags & LOST == 0 {
        return None;
    }
    ctx.first_scanout_done = false;
    ctx.damage.mark_full(ctx.width, ctx.height);
    Some("screen blanked while this compositor was silent; repainting")
}
