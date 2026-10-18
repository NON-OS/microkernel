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

//! Driving the driver found: CAPS first; an answer that is not CAPS is a
//! driver not ready (or one speaking no display wire). Otherwise SURFACE, which
//! makes this compositor the owner (a restarted compositor gets its surface
//! back), and the primary is attached, then every further output's. With
//! HW_CURSOR the pointer goes on the device's plane (`hw_cursor`).

use nonos_gfx_wire::body::caps::{HW_CURSOR, MODE_SET};

use super::attach_primary::attach_primary;
use super::discover::GfxEndpoint;
use crate::display::{ops, WireLink};
use crate::state::Context;

pub fn prime(ep: GfxEndpoint) -> Result<Context, &'static str> {
    let caps = ops::caps(ep.port).map_err(|_| "display: CAPS not answered")?;
    let s = ops::surface(ep.port, 0).map_err(|_| "display: SURFACE refused")?;
    let p = attach_primary(s.handle)?;
    if (p.width, p.height) != (s.width, s.height) {
        return Err("display: primary is not the size SURFACE said");
    }
    let mut ctx = Context::new(p, ep.port, false);
    let (name, serial, pid, physical_mm) = (ep.name, s.serial, ep.pid, caps.physical_mm);
    let outputs = caps.outputs;
    let cursor_offset = if caps.flags & HW_CURSOR != 0 { s.cursor_offset } else { 0 };
    let (stale, leaving, nudged, fixed) = (false, false, false, caps.flags & MODE_SET == 0);
    let link = WireLink {
        name,
        serial,
        pid,
        physical_mm,
        outputs,
        stale,
        cursor_offset,
        leaving,
        nudged,
        fixed,
    };
    ctx.wire = Some(link);
    ctx.heads = super::heads::prime_heads(ep.port, outputs);
    Ok(ctx)
}
