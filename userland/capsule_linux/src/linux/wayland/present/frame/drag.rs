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

//! A window moved by its title bar: held at a press, it follows the pointer
//! by what the pointer moved on screen since, so the window under it moving
//! does not move the reckoning. It stops at the menubar's foot.

use nonos_app_skeleton::clients::compositor::scene_submit;
use nonos_app_skeleton::clients::wm::window_move;
use nonos_app_skeleton::discover::lookup_port;
use nonos_app_skeleton::runner::chrome::menubar_for;

use super::super::super::scene::Scene;
use super::super::window::APP_LAYER_Z;
use super::geometry::Around;

/// A press on the title bar at (`x`, `y`) on the window.
pub fn begin(scene: &mut Scene, (x, y): (i32, i32)) {
    let Some(at) = scene.at else { return };
    scene.frame.drag = Some((at, (at.0 as i32 + x, at.1 as i32 + y)));
    let (w, h) = super::super::super::output::screen();
    scene.frame.top = menubar_for(w, h);
}

/// The pointer at (`x`, `y`) on the window while held.
pub fn to(
    scene: &mut Scene,
    (base, press): ((u32, u32), (i32, i32)),
    (x, y): (i32, i32),
    a: Around,
) {
    let Some(at) = scene.at else { return };
    let (sx, sy) = (at.0 as i32 + x, at.1 as i32 + y);
    let nx = (base.0 as i32 + sx - press.0).max(0) as u32;
    let ny = (base.1 as i32 + sy - press.1).max(scene.frame.top as i32) as u32;
    if (nx, ny) == at {
        return;
    }
    let (Some(comp), Some(wm)) = (lookup_port(b"compositor"), lookup_port(b"wm")) else { return };
    let (Some(handle), Some(id)) = (scene.out, scene.window) else { return };
    let rid = scene.next_serial();
    if scene_submit(comp, rid, handle, nx, ny, a.w, a.h, APP_LAYER_Z).is_err() {
        return;
    }
    scene.at = Some((nx, ny));
    let rid = scene.next_serial();
    let _ = window_move(wm, rid, id, nx, ny);
}
