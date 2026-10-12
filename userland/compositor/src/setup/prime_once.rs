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

use nonos_libc::SURFACE_FORMAT_ARGB8888;

use super::attach_primary::attach_primary;
use super::discover;
use crate::gfx_client;
use crate::state::Context;

pub fn run_virtio_once() -> Result<Context, &'static str> {
    let gfx = discover::lookup_gfx_endpoint()?;
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
