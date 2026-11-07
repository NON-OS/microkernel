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

//! A window minimised and shown again. The window manager keeps the window,
//! so it stays in the dock, but the compositor draws every layer it holds:
//! a window stays on screen until its owner takes its layer off, as a native
//! app does (app_skeleton runner/apply_drain.rs). So a minimise takes the
//! window's layer off the compositor and lets go of its buffers, leaving the
//! window registered afresh when it is shown again (reshape.rs).
//!
//! It is shown again when the desktop shell says the window was chosen (its
//! dock tile clicked), which reaches this capsule as the focus-self control
//! frame every app's process is sent (input/control.rs); the guest's last
//! buffer is drawn on a new surface at the window's old place.

use core::mem;

use nonos_app_skeleton::clients::compositor::scene_remove_layer;
use nonos_app_skeleton::clients::wm::{window_focus, window_raise, window_restore};
use nonos_app_skeleton::discover::lookup_port;
use nonos_toolkit::decorations::DecorationHit;

use crate::linux::guest::Guest;

use super::super::scene::Scene;
use super::window::APP_LAYER_Z;

/// The shown window hidden: its layer taken off the compositor's scene and
/// its buffers let go of, its window kept in the window manager (which the
/// caller has told to minimise it). A later restore shows it where it was.
pub fn hide(scene: &mut Scene) {
    // Already hidden, or never shown: nothing to take off.
    let Some(handle) = scene.win.out.take() else {
        scene.win.minimized = true;
        return;
    };
    let rid = scene.next_serial();
    if let Some(port) = lookup_port(b"compositor") {
        let _ = scene_remove_layer(port, rid, handle, APP_LAYER_Z);
    }
    let mut bufs = mem::take(&mut scene.win.bufs);
    let _ = bufs.release();
    // Shown again only through a fresh surface: the compositor forgot the
    // handle with the layer, so the old one names nothing (scene_buffers.rs).
    scene.win.shape = None;
    scene.win.owing = None;
    scene.win.frame.chrome = None;
    // The control the pointer lit (the minimise button that hid it) is put
    // out: no motion reaches a hidden window, so it was shown again lit.
    scene.win.frame.hover = DecorationHit::None;
    scene.win.minimized = true;
}

/// The window shown again after a minimise, at its old place: the window
/// manager told to restore, raise and focus it, and the guest's last buffer
/// drawn on a new surface (present/window.rs). A window not minimised is
/// only raised and focused, as the click asks.
pub fn restore(guest: &mut Guest) {
    let (Some(wm), Some(id)) = (lookup_port(b"wm"), guest.scene.win.window) else { return };
    let _ = window_restore(wm, guest.scene.next_serial(), id);
    let _ = window_raise(wm, guest.scene.next_serial(), id);
    let _ = window_focus(wm, guest.scene.next_serial(), id);
    if !guest.scene.win.minimized {
        return;
    }
    // Shown again before the next guest commit is let through (damage_show.rs).
    guest.scene.win.minimized = false;
    let Some(shown) = guest.scene.win.shown else { return };
    let surface = shown.surface;
    // The last committed buffer only: `pending` names an attach as soon as
    // it is made, and one not yet committed may be of another shape or still
    // being drawn. Then the commit that applies it shows the window. Either
    // way present() reads it only as its own shape, inside its pool, from its
    // memfd as mapped now (present/source.rs), so a buffer that went or
    // changed while minimised shows nothing until the next commit.
    let s = guest.scene.surfaces.iter().find(|s| s.id == surface);
    let buffer = s.filter(|s| !s.attached).and_then(|s| s.pending);
    if let Some(buffer) = buffer {
        // A buffer of the window's shape opens a new surface for it
        // (reshape.rs is Open, win.shape having been cleared on hide).
        let _ = super::show::present(guest, surface, buffer);
    }
}
