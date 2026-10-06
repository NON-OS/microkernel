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

//! Leave the firmware framebuffer for the virtio-gpu driver once it is up.
//!
//! The compositor falls back to GOP when the gfx driver has not announced
//! itself after a few setup attempts. On a multi-core machine those attempts
//! finish before init has even spawned the driver, and once the driver talks
//! to a virtio-vga device the device stops scanning out the firmware
//! framebuffer, so a compositor that stays in GOP mode draws to a buffer
//! nobody shows. While in GOP mode the server loop calls this now and then;
//! the first time the driver answers, the display moves over to it and the
//! whole scene is repainted there. Clients, layers, focus and cursor stay.

use crate::state::Context;

pub fn upgrade_to_virtio(ctx: &mut Context) -> bool {
    if !ctx.gop_mode {
        return false;
    }
    let Ok(next) = super::prime_once::run_virtio_once() else {
        return false;
    };
    super::canvas_release::release_canvas(ctx);
    ctx.gfx_port = next.gfx_port;
    ctx.resource_id = next.resource_id;
    ctx.width = next.width;
    ctx.height = next.height;
    ctx.stride = next.stride;
    ctx.backing_len = next.backing_len;
    ctx.backing_va = next.backing_va;
    ctx.gop_mode = false;
    ctx.surface_handle = next.surface_handle;
    ctx.first_scanout_done = false;
    ctx.scanout_error_reported = false;
    ctx.damage.mark_full(ctx.width, ctx.height);
    super::canvas::fit_canvas(ctx);
    true
}
