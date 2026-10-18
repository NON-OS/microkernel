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

//! The window's frame with its subsurfaces laid on it. The window's own
//! pixels are kept beside it while it has subsurfaces, so a commit that
//! changes only a subsurface repaints from them (repaint), the toplevel's
//! buffer having gone back to the client at its own commit.

use nonos_app_skeleton::clients::compositor::damage_commit;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::super::present::surface::say;
use super::super::reshape::Shape;
use super::blend::{lay, Plane};
use super::order::layers;
use super::state::Subs;

/// Lay the subsurfaces placed on `root` onto its `frame`, shaped `shape`,
/// keeping the frame as it was for a later repaint.
pub fn compose(subs: &mut Subs, root: u32, frame: &mut [u8], shape: Shape) {
    if !subs.stacks.iter().any(|s| s.parent == root) {
        subs.base = alloc::vec::Vec::new();
        return;
    }
    subs.base.clear();
    if subs.base.try_reserve_exact(frame.len()).is_err() {
        say("[WAYLAND] no room to keep the window's pixels for its subsurfaces\n".into());
    } else {
        subs.base.extend_from_slice(frame);
    }
    paint(subs, root, frame, shape);
}

/// Paint the window again from its kept pixels, as a subsurface changed.
pub fn repaint(guest: &mut Guest) {
    let scene = &mut guest.scene;
    let (Some(shown), Some(shape)) = (scene.shown.as_ref(), scene.shape) else { return };
    let root = shown.surface;
    // Kept for this shape, not one a refused show left behind.
    let len = scene.subs.base.len();
    let fits = len != 0 && len as u64 == u64::from(shape.2) * u64::from(shape.1);
    let Some(frame) = scene.pixels.frame(len).filter(|_| fits) else { return };
    frame.copy_from_slice(&scene.subs.base);
    paint(&scene.subs, root, frame, shape);
    let (Some((x, y)), Some(port)) = (scene.at, lookup_port(b"compositor")) else { return };
    let _ = damage_commit(port, scene.next_serial(), x, y, shape.0, shape.1);
}

fn paint(subs: &Subs, root: u32, frame: &mut [u8], (w, h, stride): Shape) {
    let at = |id| subs.of(id).filter(|s| s.shown.is_some()).map(|s| s.at);
    let (below, above) = layers(&subs.stacks, root, &at);
    let plane = Plane { w, h, stride };
    // Beneath the window, nearest first; above it, lowest first.
    let lays = below.iter().rev().map(|l| (l, false)).chain(above.iter().map(|l| (l, true)));
    for (&(id, xy), over) in lays {
        if let Some(k) = subs.of(id).and_then(|s| s.shown.as_ref()) {
            lay(frame, plane, &k.px, k.plane, xy, over);
        }
    }
}
