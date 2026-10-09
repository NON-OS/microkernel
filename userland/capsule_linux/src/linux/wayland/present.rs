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

//! Getting the client's pixels onto a NONOS surface.

use nonos_app_skeleton::clients::compositor::damage_commit;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::frame_len::frame_len;
use super::present_window::{refit, show};
use super::reshape::{reshape, Reshape};
use super::scene::Scene;
use super::window_damage::window_damage;

const NO_POOL: &[u8] = b"[WAYLAND] commit of a buffer with no pool behind it, not shown\n";
const NO_FRAME: &[u8] =
    b"[WAYLAND] commit of a buffer empty, outside its pool or larger than a frame, not shown\n";
const NO_ROOM: &[u8] = b"[WAYLAND] no room for the frame, not shown\n";

/// Show `buffer`, committed on `surface`.
pub fn present(guest: &mut Guest, surface: u32, buffer: u32) {
    let (at, width, height, stride, bytes) = match source(&guest.scene, buffer) {
        Ok(found) => found,
        Err(why) => return say(why),
    };
    let shape = (width, height, stride);
    // Presented by the compositor, as every other app's window is; the
    // personality holds no GfxPresent and needs none. A buffer of a shape
    // the surface does not have gets one that has it (reshape.rs).
    if reshape(guest.scene.shape, shape) != Reshape::Same {
        return show(guest, surface, at, bytes, shape);
    }
    /* Straight into the frame: a second buffer would hold the pixels twice. */
    let pid = guest.pid;
    let Some(frame) = guest.scene.pixels.frame(bytes) else {
        return say(NO_ROOM);
    };
    if !Guest::read_into(pid, at, frame) {
        return;
    }
    if guest.scene.fit.due().is_some() {
        refit(&mut guest.scene);
    }
    // In screen coordinates, where the window is (window_damage.rs).
    let Some((x, y, w, h)) = window_damage(guest.scene.at, width, height) else {
        return;
    };
    if let Some(port) = lookup_port(b"compositor") {
        let _ = damage_commit(port, guest.scene.next_serial(), x, y, w, h);
    }
}

/// Where the pixels are, how they are shaped, and how many bytes that is
/// (`frame_len`), or what to say instead.
fn source(scene: &Scene, buffer: u32) -> Result<(u64, u32, u32, u32, usize), &'static [u8]> {
    let b = scene.buffers.iter().find(|b| b.id == buffer).ok_or(NO_POOL)?;
    let pool = scene.pools.iter().find(|p| p.id == b.pool).ok_or(NO_POOL)?;
    let at = pool.at.checked_add(b.offset).ok_or(NO_FRAME)?;
    let bytes = frame_len(b.offset, b.stride, b.height, pool.size).ok_or(NO_FRAME)?;
    Ok((at, b.width, b.height, b.stride, bytes))
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
