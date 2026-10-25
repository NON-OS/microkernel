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

//! A guest window opened with the window manager the first time its surface
//! is shown, so it is raised, focused and given the keys.

use nonos_app_skeleton::clients::wm::window_open;
use nonos_libc::mk_surface_release;

use super::super::scene::Scene;
use super::surface::say;

/// Window ids for guest windows, clear of the native apps' fixed ones.
const GUEST_WINDOWS: u32 = 0x4C4E_0000;

/// The window's id when this opened it, None when it was open already;
/// Err when the window manager refused, the surface `handle` released.
/// `at` becomes the window manager's placement unless a configure is due.
pub(super) fn open_once(
    scene: &mut Scene,
    wm: u32,
    handle: u64,
    at: &mut (u32, u32),
    due: bool,
    (w, h): (u32, u32),
) -> Result<Option<u32>, ()> {
    if scene.window.is_some() {
        return Ok(None);
    }
    let id = GUEST_WINDOWS | (handle as u32 & 0xFFFF);
    match window_open(wm, scene.next_serial(), id, 0, at.0, at.1, w, h) {
        Ok(p) => {
            // A configure's corner is the guest's own; else the
            // window manager's placement stands.
            if !due {
                *at = (p.x, p.y);
            }
            Ok(Some(id))
        }
        Err(why) => {
            let _ = mk_surface_release(handle);
            say(alloc::format!("[WAYLAND] window not opened: {why}\n"));
            Err(())
        }
    }
}
