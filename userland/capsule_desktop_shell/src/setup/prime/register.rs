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

use nonos_libc::{
    mk_surface_register, mk_surface_release, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

use super::overlay::Overlay;
use crate::compositor_client::push_scene_submit;

/// The desk's band: over the wallpaper's (0), under every application
/// window's (2).
pub const DESK_Z: u32 = 1;

/// The chrome's band: over every application window, under the boot splash
/// (4_000_000), which covers the whole screen while the system starts.
///
/// The shell used to hold one surface only, in the desk's band, so its menus,
/// the Launchpad, the right-click menu, toasts and consent dialogs all drew
/// under any window open where they opened, while their clicks were still
/// meant for them. The compositor keeps one layer per process and band, so
/// the shell holds both without anything new being granted to it.
pub const CHROME_Z: u32 = 3_000_000;

pub fn register_overlay(
    compositor_port: u32,
    request_id: u32,
    overlay: &Overlay,
    z: u32,
) -> Result<u64, &'static str> {
    let desc = SurfaceDescriptor {
        width: overlay.width,
        height: overlay.height,
        stride: overlay.stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: overlay.byte_len,
        base_va: overlay.backing_va,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        return Err("overlay surface register rejected");
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        return Err("overlay surface share rejected");
    }
    if let Err(e) = push_scene_submit(
        compositor_port,
        request_id,
        handle as u64,
        0,
        0,
        overlay.width,
        overlay.height,
        z,
    ) {
        if mk_surface_release(handle as u64) < 0 {
            return Err("overlay surface release rejected");
        }
        return Err(e);
    }
    Ok(handle as u64)
}
