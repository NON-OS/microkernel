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

use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::super::fit::origin;
use super::super::subsurface::compose::compose;
use super::adopt::{adopt, Placed};
use super::open::open_once;
use super::place::place;
use super::refit::tell_wm;
use super::source::{taken, Source};
use super::surface::{register, say};

/// Native app windows' layer; 0 is the wallpaper's, under the desktop.
pub(super) const APP_LAYER_Z: u32 = 2;

/// Show the `shape` buffer at `src` in the guest, `bytes` long, committed
/// on `surface`, on a new surface of its shape: opened, raised and focused
/// the first time, at the acked configure's corner when one is due, else
/// where the window was, else centred below the top bar.
pub fn show(guest: &mut Guest, surface: u32, src: &Source) {
    let shape = src.shape;
    let (Some(comp), Some(wm)) = (lookup_port(b"compositor"), lookup_port(b"wm")) else {
        say("[WAYLAND] no compositor or window manager for a window\n".into());
        return;
    };
    let Some(mut fresh) = taken(guest.pid, src) else { return };
    if let Some(frame) = fresh.frame(src.bytes) {
        compose(&mut guest.scene.subs, surface, frame, shape);
    }
    let Some(handle) = register(&fresh, shape) else { return };
    let scene = &mut guest.scene;
    let (w, h, _) = shape;
    let due = scene.fit.due();
    let mut at = origin(due, scene.at, w, h);
    let Ok(opened) = open_once(scene, wm, handle, &mut at, due.is_some(), (w, h)) else {
        return;
    };
    let Some(id) = scene.window.or(opened) else { return };
    let opened = opened.is_some();
    if !place(scene, (comp, wm), handle, at, (w, h), id, opened) {
        return;
    }
    adopt(scene, wm, Placed { surface, fresh, handle, shape, at, id, opened });
    tell_wm(scene, wm, id, !opened);
}
