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

//! Following the host (G 6, section 12 Size), on the poll: at start the
//! preferred mode, then any size the host has wanted for 250 ms, put
//! through `fit` against CAPS and MODES; one MODE_SET, then the new primary
//! adopted at once, so only one is ever in flight and the canvas refits
//! (`fit_canvas`) before the next frame. A refusal keeps the old mode.

use nonos_gfx_wire::body::caps::MODE_SET;
use nonos_gfx_wire::body::mode::PREFERRED;
use nonos_gfx_wire::fit::fit;
use nonos_gfx_wire::State;
use nonos_libc::mk_uptime_ms;

use super::discover::GfxEndpoint;
use crate::display::follow::FOLLOW_HOST;
use crate::display::{ops, ops_mode};
use crate::state::Context;

pub fn follow(ctx: &mut Context, ep: GfxEndpoint, state: Option<State>) -> Option<&'static str> {
    if !FOLLOW_HOST || ctx.wire.is_none() {
        return None;
    }
    let (now, start) = (mk_uptime_ms(), !ctx.resize.started);
    if let Some(s) = state {
        ctx.resize.note((s.hint_width, s.hint_height), now);
    }
    // CAPS and MODES are asked only when there is something to fit.
    if !start && !ctx.resize.due(now) {
        return None;
    }
    ctx.resize.started = true;
    let caps = ops::caps(ep.port).ok().filter(|c| c.flags & MODE_SET != 0)?;
    let modes = ops_mode::modes(ep.port, 0).ok()?;
    if start {
        let p = modes.as_slice().iter().find(|m| m.flags & PREFERRED != 0);
        ctx.resize.start(p.map(|m| (m.width, m.height)));
    }
    let want = ctx.resize.take(now)?;
    let size = fit(want, (ctx.screen.width, ctx.screen.height), &caps, modes.as_slice())?;
    if ops_mode::mode_set(ep.port, 0, size).is_err() {
        return Some("the host's size refused; the old mode stays");
    }
    let next = super::prime_wire::prime(ep).ok()?;
    super::adopt::adopt(ctx, next);
    Some("resized to the host's size")
}
