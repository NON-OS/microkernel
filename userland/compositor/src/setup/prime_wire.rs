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

//! Driving the driver found: CAPS first. ENOSYS is a driver that speaks only
//! the legacy ops (0x0C, then 0x08/0x09/0x0A each frame); any other answer
//! to CAPS that is not CAPS is a driver not ready. Otherwise SURFACE, which
//! makes this compositor the owner (a restarted compositor gets its surface
//! back), and the primary is attached.

use nonos_gfx_wire::errno::ENOSYS;

use super::attach_primary::attach_primary;
use super::discover::GfxEndpoint;
use crate::display::{ops, WireLink};
use crate::state::Context;

pub fn prime(ep: GfxEndpoint) -> Result<Context, &'static str> {
    let caps = match ops::caps(ep.port) {
        Err(ENOSYS) => return super::prime_once::run_legacy(ep),
        Err(_) => return Err("display: CAPS not answered"),
        Ok(c) => c,
    };
    let s = ops::surface(ep.port).map_err(|_| "display: SURFACE refused")?;
    let p = attach_primary(s.handle)?;
    if (p.width, p.height) != (s.width, s.height) {
        return Err("display: primary is not the size SURFACE said");
    }
    let mut ctx = Context::new(p, ep.port, 0, false);
    ctx.first_scanout_done = true;
    let (name, serial, pid, physical_mm) = (ep.name, s.serial, ep.pid, caps.physical_mm);
    ctx.wire = Some(WireLink { name, serial, pid, physical_mm, stale: false });
    Ok(ctx)
}
