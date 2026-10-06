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

use nonos_libc::{mk_munmap, mk_surface_release};

use crate::clients::wm::WindowPlacement;
use crate::discover::Peers;

use super::backing::alloc_backing;
use super::binding::WindowBinding;
use super::register::register_and_share;
use super::submit_scene::submit_scene;

/// Give the window a new surface of `w` by `h` at (`x`, `y`), drawn by
/// `paint` before the compositor is shown it.
///
/// This used to take the window out of the scene first and then submit the
/// new surface still blank (transparent), with the paint only after: every
/// maximize, restore and step of a resize drag showed the desktop where the
/// window was for a frame or more. The new surface is now painted first and
/// submitted in the old layer's place (the compositor replaces a client's
/// layer in its band and repaints the old and new rectangles in one frame),
/// and only then is the old surface let go. When the submit is refused the
/// old surface stays on screen and the new one is given back.
pub fn reopen_surface(
    peers: &Peers,
    old: &WindowBinding,
    placement: WindowPlacement,
    request_id: &mut u32,
    paint: impl FnOnce(&WindowBinding),
) -> Result<WindowBinding, &'static str> {
    let WindowPlacement { x, y, width: w, height: h } = placement;
    let (backing_va, stride, byte_len) = alloc_backing(w, h)?;
    let surface_handle = match register_and_share(backing_va, w, h, stride, byte_len) {
        Ok(handle) => handle,
        Err(e) => {
            let _ = mk_munmap(backing_va as *mut u8, byte_len as usize);
            return Err(e);
        }
    };
    let binding = WindowBinding {
        surface_handle,
        backing_va,
        x,
        y,
        width: w,
        height: h,
        stride_words: w,
        byte_len,
    };
    paint(&binding);
    if let Err(e) = submit_scene(peers, surface_handle, request_id, placement) {
        let _ = mk_surface_release(surface_handle);
        let _ = mk_munmap(backing_va as *mut u8, byte_len as usize);
        return Err(e);
    }
    let _ = mk_surface_release(old.surface_handle);
    let _ = mk_munmap(old.backing_va as *mut u8, old.byte_len as usize);
    Ok(binding)
}
