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

//! A guest's surface made a window: opened with the window manager, as
//! every native app's is, so it is raised and focused and the keys typed
//! into it reach the guest. A surface only placed with the compositor was
//! drawn but never focused, and the input router sends keys to focus.

use nonos_app_skeleton::clients::compositor::scene_submit;
use nonos_app_skeleton::clients::wm::{window_focus, window_open, window_raise};
use nonos_app_skeleton::discover::lookup_port;

use super::present_surface::say;

/// Native app windows' layer; 0 is the wallpaper's, under the desktop.
const APP_LAYER_Z: u32 = 2;
/// Window ids for guest windows, clear of the native apps' fixed ones.
const GUEST_WINDOWS: u32 = 0x4C4E_0000;

/// Open, place, raise and focus the window for `handle`, centred below the
/// top bar unless the window manager places it elsewhere.
pub fn place(handle: u64, width: u32, height: u32) {
    let (Some(comp), Some(wm)) = (lookup_port(b"compositor"), lookup_port(b"wm")) else {
        return say("[WAYLAND] no compositor or window manager for a window\n".into());
    };
    let id = GUEST_WINDOWS | (handle as u32 & 0xFFFF);
    let x = 1920u32.saturating_sub(width) / 2;
    let y = 1080u32.saturating_sub(height).saturating_sub(48) / 2 + 48;
    let at = match window_open(wm, 1, id, 0, x, y, width, height) {
        Ok(p) => (p.x, p.y),
        Err(why) => return say(alloc::format!("[WAYLAND] window not opened: {why}\n")),
    };
    if let Err(why) = scene_submit(comp, 2, handle, at.0, at.1, width, height, APP_LAYER_Z) {
        return say(alloc::format!("[WAYLAND] window {width}x{height} not placed: {why}\n"));
    }
    let _ = window_raise(wm, 3, id);
    let focused = window_focus(wm, 4, id).is_ok();
    say(alloc::format!(
        "[WAYLAND] window {width}x{height} at {},{}, focused {focused}\n",
        at.0,
        at.1
    ));
}
