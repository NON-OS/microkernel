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

//! The legacy path, for a driver that answers CAPS with ENOSYS (today's
//! virtio-gpu): its primary through GET_PRIMARY_SURFACE.

use nonos_libc::SURFACE_FORMAT_ARGB8888;

use super::attach_primary::attach_primary;
use super::discover::{self, GfxEndpoint};
use crate::gfx_client;
use crate::state::Context;

/// The first announced driver, on whichever path it speaks.
pub fn run_display_once() -> Result<Context, &'static str> {
    super::prime_wire::prime(discover::lookup_gfx_endpoint()?)
}

pub fn run_legacy(gfx: GfxEndpoint) -> Result<Context, &'static str> {
    let primary = gfx_client::get_primary_surface(gfx.port, 1)?;
    if primary.handle == 0 || primary.width == 0 || primary.height == 0 {
        return Err("gfx primary surface absent");
    }
    if primary.format != SURFACE_FORMAT_ARGB8888 {
        return Err("gfx primary surface format mismatch");
    }
    let p = attach_primary(primary.handle)?;
    Ok(Context::new(p, gfx.port, primary.resource_id, false))
}
