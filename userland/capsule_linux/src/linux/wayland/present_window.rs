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
//!
//! A buffer of another shape gets a surface of its own (reshape.rs), shown
//! in one submit that repaints the old rect and the new, and the window
//! manager is told the window's new rect: a full-screen one with its
//! full-screen flag, so the dock hides (fit.rs).

use core::mem;

use nonos_app_skeleton::clients::compositor::scene_submit;
use nonos_app_skeleton::clients::wm::{
    window_close, window_focus, window_maximize, window_open, window_raise, window_resize,
};
use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::mk_surface_release;

use crate::linux::guest::Guest;

use super::fit::origin;
use super::present_surface::{register, say};
use super::reshape::Shape;
use super::scene::Scene;
use super::scene_pixels::Pixels;
use super::window_life::Shown;

/// Native app windows' layer; 0 is the wallpaper's, under the desktop.
const APP_LAYER_Z: u32 = 2;
/// Window ids for guest windows, clear of the native apps' fixed ones.
const GUEST_WINDOWS: u32 = 0x4C4E_0000;

/// Show the `shape` buffer at `src` in the guest, `bytes` long, committed
/// on `surface`, on a new surface of its shape: opened, raised and focused
/// the first time, at the acked configure's corner when one is due, else
/// where the window was, else centred below the top bar.
pub fn show(guest: &mut Guest, surface: u32, src: u64, bytes: usize, shape: Shape) {
    let (Some(comp), Some(wm)) = (lookup_port(b"compositor"), lookup_port(b"wm")) else {
        say("[WAYLAND] no compositor or window manager for a window\n".into());
        return;
    };
    let Some(mut fresh) = Pixels::with_room(bytes) else {
        say(alloc::format!("[WAYLAND] no room for a {bytes} byte frame, not shown\n"));
        return;
    };
    let pid = guest.pid;
    let Some(frame) = fresh.frame(bytes) else { return };
    if !Guest::read_into(pid, src, frame) {
        return;
    }
    let Some(handle) = register(&fresh, shape) else { return };
    let scene = &mut guest.scene;
    let (w, h, _) = shape;
    let due = scene.fit.due();
    let mut at = origin(due, scene.at, w, h);
    let opened = match scene.window {
        Some(_) => None,
        None => {
            let id = GUEST_WINDOWS | (handle as u32 & 0xFFFF);
            match window_open(wm, scene.next_serial(), id, 0, at.0, at.1, w, h) {
                Ok(p) => {
                    // A configure's corner is the guest's own; else the
                    // window manager's placement stands.
                    if due.is_none() {
                        at = (p.x, p.y);
                    }
                    Some(id)
                }
                Err(why) => {
                    let _ = mk_surface_release(handle);
                    say(alloc::format!("[WAYLAND] window not opened: {why}\n"));
                    return;
                }
            }
        }
    };
    let Some(id) = scene.window.or(opened) else { return };
    let rid = scene.next_serial();
    if let Err(why) = scene_submit(comp, rid, handle, at.0, at.1, w, h, APP_LAYER_Z) {
        let _ = mk_surface_release(handle);
        if opened.is_some() {
            let _ = window_close(wm, scene.next_serial(), id);
        }
        say(alloc::format!("[WAYLAND] window {w}x{h} not placed: {why}\n"));
        return;
    }
    let old_pixels = mem::replace(&mut scene.pixels, fresh);
    let old = scene.out.replace(handle);
    scene.shape = Some(shape);
    scene.at = Some(at);
    scene.window = Some(id);
    let toplevel = scene.surfaces.iter().find(|s| s.id == surface).and_then(|s| s.toplevel);
    scene.shown = Some(Shown { surface, toplevel });
    if let Some(old) = old {
        let_go(old, old_pixels);
    }
    if opened.is_some() {
        let _ = window_raise(wm, scene.next_serial(), id);
        let focused = window_focus(wm, scene.next_serial(), id).is_ok();
        say(alloc::format!("[WAYLAND] window {w}x{h} at {},{}, focused {focused}\n", at.0, at.1));
    }
    tell_wm(scene, wm, id, opened.is_none());
}

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
fn tell_wm(scene: &mut Scene, wm: u32, id: u32, resized: bool) {
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

/// The old surface released, then its pixels freed. Pixels the kernel would
/// not let go of are kept, since it may still map them.
fn let_go(handle: u64, pixels: Pixels) {
    let rc = mk_surface_release(handle);
    if rc < 0 {
        mem::forget(pixels);
        say(alloc::format!("[WAYLAND] surface {handle} not released, rc {rc}; its pixels kept\n"));
    }
}
