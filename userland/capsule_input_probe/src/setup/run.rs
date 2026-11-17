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

//! Bringing the probe up: its services found, its surface made and drawn, and
//! the compositor showing it.

use nonos_libc::mk_surface_release;

use super::{discover, surface};
use crate::clients::{compositor, display_info};
use crate::render;
use crate::state::Context;

const OVERLAY_Z: u32 = 1;

pub fn run() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    let router_port = discover::lookup_router_port()?;
    compositor::healthcheck(compositor_port, 1).map_err(|_| "compositor health failed")?;
    let di = display_info::query_display_info(compositor_port, 3)
        .map_err(|_| "display info query failed")?;
    let (width, height, stride) = (di.width, di.height, di.stride);
    let (backing_va, handle) = surface::make(width, height, stride)?;
    let ctx = Context::new(backing_va, width, height, stride, compositor_port, router_port);
    render::redraw(&ctx);
    let submit = compositor::push_scene_submit(
        compositor_port,
        1,
        handle as u64,
        0,
        0,
        width,
        height,
        OVERLAY_Z,
    );
    if submit.is_err() {
        let _ = mk_surface_release(handle as u64);
        return Err("compositor scene submit failed");
    }
    if compositor::damage_commit(compositor_port, 2, width, height).is_err() {
        let _ = mk_surface_release(handle as u64);
        return Err("compositor damage commit failed");
    }
    Ok(ctx)
}
