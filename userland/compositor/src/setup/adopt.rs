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

//! Moving the desktop onto a new primary: from GOP to a driver, onto a
//! restarted driver, or onto a primary the driver replaced. Clients,
//! layers, focus and cursor stay; the whole scene is repainted there.
//!
//! In section 12's order: the new primary is attached (by the caller),
//! swapped in, the old handle released, then RETIRE sent for the old serial
//! when the same driver served both, so the driver frees it only once
//! nothing here can touch it.

use nonos_libc::mk_surface_release;

use crate::display::ops;
use crate::state::Context;

pub fn adopt(ctx: &mut Context, next: Context) {
    super::canvas_release::release_canvas(ctx);
    let (old_handle, old_gop) = (ctx.surface_handle, ctx.gop_mode);
    let old = ctx.wire.map(|w| (ctx.gfx_port, w.pid, w.serial));
    ctx.gfx_port = next.gfx_port;
    ctx.resource_id = next.resource_id;
    ctx.width = next.width;
    ctx.height = next.height;
    ctx.stride = next.stride;
    ctx.backing_len = next.backing_len;
    ctx.backing_va = next.backing_va;
    ctx.gop_mode = next.gop_mode;
    ctx.surface_handle = next.surface_handle;
    ctx.first_scanout_done = next.first_scanout_done;
    ctx.scanout_error_reported = false;
    ctx.wire = next.wire;
    ctx.damage.mark_full(ctx.width, ctx.height);
    super::canvas::fit_canvas(ctx);
    // GOP's surface is the compositor's own registration, kept as before.
    if !old_gop && old_handle != ctx.surface_handle {
        let _ = mk_surface_release(old_handle);
    }
    if let (Some((port, pid, serial)), Some(new)) = (old, ctx.wire) {
        // The current serial answers EINVAL; a restarted driver has
        // already let go of the old one with its process.
        if pid == new.pid && serial != new.serial {
            let _ = ops::retire(port, serial);
        }
    }
}
