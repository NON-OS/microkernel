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

//! A guest window's surface placed with the compositor, and undone when the
//! compositor refuses it.

use nonos_app_skeleton::clients::compositor::scene_submit;
use nonos_app_skeleton::clients::wm::window_close;
use nonos_libc::mk_surface_release;

use super::super::scene::Scene;
use super::surface::say;
use super::window::APP_LAYER_Z;

/// Place surface `handle` at `at` as window `id`. On a refusal the surface
/// is released and a window this show `opened` closed, and it is false.
pub(super) fn place(
    scene: &mut Scene,
    (comp, wm): (u32, u32),
    handle: u64,
    at: (u32, u32),
    (w, h): (u32, u32),
    id: u32,
    opened: bool,
) -> bool {
    let rid = scene.next_serial();
    let Err(why) = scene_submit(comp, rid, handle, at.0, at.1, w, h, APP_LAYER_Z) else {
        return true;
    };
    let _ = mk_surface_release(handle);
    if opened {
        let _ = window_close(wm, scene.next_serial(), id);
    }
    say(alloc::format!("[WAYLAND] window {w}x{h} not placed: {why}\n"));
    false
}
