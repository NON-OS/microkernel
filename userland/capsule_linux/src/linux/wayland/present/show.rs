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

use super::super::reshape::{reshape, Reshape};
use super::super::subsurface::compose::compose;
use super::super::window_damage::window_damage;
use super::refit::refit;
use super::source::{source, take};
use super::window::show;

const NO_ROOM: &[u8] = b"[WAYLAND] no room for the frame, not shown\n";

/// Show `buffer`, committed on `surface`.
pub fn present(guest: &mut Guest, surface: u32, buffer: u32) {
    let src = match source(&guest.scene, buffer) {
        Ok(found) => found,
        Err(why) => return say(why),
    };
    let (width, height, _) = src.shape;
    // Presented by the compositor, as every other app's window is; the
    // personality holds no GfxPresent and needs none. A buffer of a shape
    // the surface does not have gets one that has it (reshape.rs).
    if reshape(guest.scene.shape, src.shape) != Reshape::Same {
        super::super::popup::ends::end_all(guest);
        return show(guest, surface, &src);
    }
    /* Straight into the frame: a second buffer would hold the pixels twice. */
    let pid = guest.pid;
    let Some(frame) = guest.scene.pixels.frame(src.bytes) else {
        return say(NO_ROOM);
    };
    if !take(pid, &src, frame) {
        return;
    }
    compose(&mut guest.scene.subs, surface, frame, src.shape);
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

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
