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

use nonos_app_skeleton::clients::compositor::scanout_peer;
use nonos_app_skeleton::discover::{lookup_port, lookup_service};

use crate::linux::guest::Guest;

use super::super::scene_buffers::Buffers;
use super::adopt::{adopt, Placed};
use super::draw::Drew;
use super::frame::fresh::fresh;
use super::open::open_once;
use super::place::place;
use super::refit::tell_wm;
use super::source::Source;
use super::surface::say;

/// Native app windows' layer; 0 is the wallpaper's, under the desktop.
pub(in crate::linux::wayland) const APP_LAYER_Z: u32 = 2;

/// Show `src`, committed on `surface`, on new buffers of the window's
/// shape: opened, raised and focused the first time, at an acked
/// configure's corner, else where it was, else centred below the top bar.
/// Buffers not placed are let go of as they drop.
pub fn show(guest: &mut Guest, surface: u32, src: &Source) -> Drew {
    let shape = src.shape;
    let (Some(comp), Some(wm)) = (lookup_port(b"compositor"), lookup_port(b"wm")) else {
        say("[WAYLAND] no compositor or window manager for a window\n".into());
        return Drew::Nothing;
    };
    super::frame::now::note_scale(&mut guest.scene);
    guest.scene.win.origin = src.origin;
    let a = super::frame::now::around_shape(&guest.scene, shape);
    let Some((px, outer)) = fresh(&mut guest.scene, (guest.pid, surface), src, a) else {
        return Drew::Nothing;
    };
    let peers = (lookup_service(b"compositor").map_or(0, |p| p.pid), driver(comp));
    // Unframed, every pixel is laid over the frame's ground: opaque, so a
    // full-screen one may be scanned out as it is.
    let Ok(bufs) = Buffers::register(px, outer, peers, APP_LAYER_Z, !a.framed()) else {
        return Drew::Nothing;
    };
    let handle = bufs.handle(0);
    let scene = &mut guest.scene;
    let due = scene.win.fit.due();
    let mut at = scene.win.fit.corner(scene.win.at, a.w, a.h);
    let Ok(opened) = open_once(scene, wm, &mut at, due.is_some(), (a.w, a.h)) else {
        return Drew::Nothing;
    };
    let Some(id) = scene.win.window.or(opened) else { return Drew::Nothing };
    let opened = opened.is_some();
    if opened {
        crate::linux::start_say::clock::window();
    }
    if !place(scene, (comp, wm), handle, at, (a.w, a.h), id, opened) {
        return Drew::Nothing;
    }
    adopt(scene, wm, Placed { surface, bufs, shape, at, id, opened });
    tell_wm(scene, wm, id, !opened);
    Drew::Shown
}

/// The display driver the buffers may be shared with for scanout, 0 when
/// the compositor names none. Asked under a request id of its own, as the
/// display's size is (toplevel/ask.rs): the scene's serials are the
/// configures' and are not spent on it.
fn driver(comp: u32) -> u32 {
    scanout_peer(comp, 1).unwrap_or(0)
}
