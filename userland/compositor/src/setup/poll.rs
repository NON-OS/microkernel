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

//! The 500 ms look at a wire driver (section 12): its service looked up
//! again, its STATE read, and the step `poll_rule` names taken, then the
//! mirrored outputs' (`poll_heads`). A FLUSH that found its primary stale
//! is answered here too, on the next pass of the loop, not mid-frame. A
//! steady driver is also where the host's size is followed (`resize`).

use crate::display::ops;
use crate::display::poll_rule::{step, Step};
use crate::state::Context;

use super::discover::{lookup, GfxEndpoint};

/// What changed, for the log; None when nothing did.
pub fn poll(ctx: &mut Context) -> Option<&'static str> {
    let link = ctx.wire?;
    ctx.wire.as_mut()?.nudged = false;
    let ep = lookup(link.name);
    let same = ep.filter(|e| e.pid == link.pid);
    let state = same.and_then(|e| ops::state(e.port, 0).ok());
    match step(&link, ep.map(|e| e.pid), state) {
        Step::Keep => {
            let e = same?;
            let resized = super::resize::follow(ctx, e, state);
            resized.or_else(|| super::poll_heads::poll_heads(ctx, e.port))
        }
        Step::Repaint => {
            ctx.damage.mark_full(ctx.width, ctx.height);
            // A lost output may have lost its cursor plane too.
            ctx.cursor_sent = None;
            let _ = same.map(|e| super::poll_heads::poll_heads(ctx, e.port));
            Some("output lost and set again; repainting")
        }
        Step::Resurface => again(ctx, ep?, "primary replaced"),
        Step::Reprime => again(ctx, ep?, "driver restarted"),
        Step::Move => super::move_off::move_off(ctx, link.pid),
    }
}

fn again(ctx: &mut Context, ep: GfxEndpoint, what: &'static str) -> Option<&'static str> {
    // CAPS again too: a restarted driver may say other things of itself.
    let next = super::prime_wire::prime(ep).ok()?;
    super::adopt::adopt(ctx, next);
    Some(what)
}
