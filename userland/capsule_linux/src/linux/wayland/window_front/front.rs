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
//! Which window a toplevel's commit goes to: its own, or a new one.

use core::mem;

use crate::linux::guest::Guest;

use super::super::scene::Scene;

/// Where a toplevel's buffer commit goes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Front {
    /// Its own window, brought to `Scene.win`, or the guest's first.
    Shown,
    /// A toplevel mapped for the first time while others are: a new window
    /// of its own, opened beside them.
    New,
    /// One more than MOST_WINDOWS: not shown, so one client cannot hold the
    /// window manager, the compositor and this capsule's heap with windows.
    Full,
}

/// The windows one guest has on screen at once. Each holds a window manager
/// window, a compositor surface and up to MAX_FRAME of pixels; a client
/// with this many open is no desktop application.
pub const MOST_WINDOWS: usize = 32;

/// Where `surface`'s commit goes, its window brought to `Scene.win`.
pub fn front(guest: &mut Guest, surface: u32) -> Front {
    let shown = guest.scene.win.shown.map(|s| s.surface);
    if shown.is_none() {
        said(&mut guest.scene, surface);
        return Front::Shown;
    }
    if super::select(guest, |w| w.shown.is_some_and(|s| s.surface == surface)) {
        return Front::Shown;
    }
    if guest.scene.behind.len() + 1 >= MOST_WINDOWS {
        return Front::Full;
    }
    let scene = &mut guest.scene;
    // The display's scale is the guest's other windows'.
    let quarters = scene.win.frame.quarters;
    let old = mem::take(&mut scene.win);
    scene.behind.push(old);
    scene.win.frame.quarters = quarters;
    said(scene, surface);
    Front::New
}

/// The title and frame `surface`'s toplevel said, taken by the window's
/// frame, which is painted afresh when they changed.
pub(super) fn said(scene: &mut Scene, surface: u32) {
    let Some(s) = scene.surfaces.iter().find(|s| s.id == surface) else { return };
    let frame = &mut scene.win.frame;
    if frame.title != s.title || frame.own_frame != s.own_frame {
        frame.title.clone_from(&s.title);
        frame.own_frame = s.own_frame;
        frame.chrome = None;
    }
}
