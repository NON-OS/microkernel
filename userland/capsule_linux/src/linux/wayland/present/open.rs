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

use super::super::scene::Scene;
use super::surface::say;

/// The window's id when this opened it, None when it was open already;
/// Err when the window manager refused; the caller lets the buffers go.
/// `at` becomes the window manager's placement unless a configure is due.
pub(super) fn open_once(
    scene: &mut Scene,
    wm: u32,
    at: &mut (u32, u32),
    due: bool,
    (w, h): (u32, u32),
) -> Result<Option<u32>, ()> {
    if scene.win.window.is_some() {
        return Ok(None);
    }
    // One no other window of this guest has (window_front/ids.rs).
    let used = super::super::window_front::windows(scene);
    let ids = super::super::window_front::ids::GUEST_WINDOWS;
    let Some(id) = super::super::window_front::ids::free_id(ids, &used) else {
        say(alloc::string::String::from("[WAYLAND] window not opened: no window id free\n"));
        return Err(());
    };
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
            say(alloc::format!("[WAYLAND] window not opened: {why}\n"));
            Err(())
        }
    }
}
