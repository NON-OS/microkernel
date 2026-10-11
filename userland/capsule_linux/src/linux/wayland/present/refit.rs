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

//! A window moved to an acked configure's corner, and the window manager
//! told the window's rect.

use nonos_app_skeleton::clients::compositor::scene_submit;
use nonos_app_skeleton::clients::wm::{window_maximize, window_resize};
use nonos_app_skeleton::discover::lookup_port;

use super::super::fit::origin;
use super::super::scene::Scene;
use super::surface::say;
use super::window::APP_LAYER_Z;

/// A buffer of the surface's own shape drawn for an acked configure: the
/// window goes to that configure's corner, and the window manager is told.
pub fn refit(scene: &mut Scene) {
    let (Some(comp), Some(wm)) = (lookup_port(b"compositor"), lookup_port(b"wm")) else {
        say("[WAYLAND] no compositor or window manager for a window\n".into());
        return;
    };
    let (Some(handle), Some(id), Some((w, h, _))) = (scene.out, scene.window, scene.shape) else {
        return;
    };
    let at = origin(scene.fit.due(), scene.at, w, h);
    if scene.at != Some(at) {
        let rid = scene.next_serial();
        if let Err(why) = scene_submit(comp, rid, handle, at.0, at.1, w, h, APP_LAYER_Z) {
            say(alloc::format!("[WAYLAND] window {w}x{h} not moved: {why}\n"));
            return;
        }
        scene.at = Some(at);
    }
    tell_wm(scene, wm, id, false);
}

/// The window manager's rect for the window: a configure's, with its flag,
/// or a size the guest took on its own (`resized`).
pub(super) fn tell_wm(scene: &mut Scene, wm: u32, id: u32, resized: bool) {
    let Some(rect) = scene.rect() else { return };
    let rid = scene.next_serial();
    let told = match scene.fit.due() {
        Some(c) => {
            scene.fit.shown();
            window_maximize(wm, rid, id, rect, c.mode.fills())
        }
        None if resized => window_resize(wm, rid, id, rect.2, rect.3),
        None => Ok(()),
    };
    if let Err(why) = told {
        say(alloc::format!(
            "[WAYLAND] window manager not told the window is {}x{} at {},{}: {why}\n",
            rect.2,
            rect.3,
            rect.0,
            rect.1
        ));
    }
}
