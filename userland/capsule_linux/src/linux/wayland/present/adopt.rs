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

//! A new surface taken as the window's: its pixels, rect and window kept in
//! the scene, the old surface let go of, and a window just opened raised
//! and focused.

use core::mem;

use nonos_app_skeleton::clients::wm::{window_focus, window_raise};

use super::super::scene::Scene;
use super::super::scene_buffers::Buffers;
use super::super::window_life::Shown;
use super::surface::say;

/// What `show` placed: the surface, its buffers (the first shown), their
/// shape, where it is, the window's id, and whether this show opened it.
pub(super) struct Placed {
    pub(super) surface: u32,
    pub(super) bufs: Buffers,
    pub(super) shape: (u32, u32, u32),
    pub(super) at: (u32, u32),
    pub(super) id: u32,
    pub(super) opened: bool,
}

/// The placed buffers made the window's, in the scene and with the window
/// manager. The old pair is let go of: the compositor's layer no longer
/// names either, and one it still held is released by its frame.
pub(super) fn adopt(scene: &mut Scene, wm: u32, p: Placed) {
    let ((w, h, _), at, id) = (p.shape, p.at, p.id);
    scene.win.out = Some(p.bufs.handle(0));
    let mut old = mem::replace(&mut scene.win.bufs, p.bufs);
    scene.win.shape = Some(p.shape);
    scene.win.at = Some(at);
    scene.win.window = Some(id);
    // The new buffers show the guest's buffer whole: nothing is owed.
    scene.win.owing = None;
    let surface = p.surface;
    let toplevel = scene.surfaces.iter().find(|s| s.id == surface).and_then(|s| s.toplevel);
    scene.win.shown = Some(Shown { surface, toplevel });
    if !old.release() {
        say("[WAYLAND] old buffers not all released; their pixels kept\n".into());
    }
    if p.opened {
        let _ = window_raise(wm, scene.next_serial(), id);
        let focused = window_focus(wm, scene.next_serial(), id).is_ok();
        say(alloc::format!("[WAYLAND] window {w}x{h} at {},{}, focused {focused}\n", at.0, at.1));
    }
}
