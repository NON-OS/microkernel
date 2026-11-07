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
//! A window brought to `Scene.win`, to be acted on; the one there goes
//! back among the others.

use core::mem;

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::scene::Scene;
use super::super::scene_window::Window;

/// The window `pick` chooses, brought to `Scene.win`; false when none is.
pub fn select(guest: &mut Guest, pick: impl Fn(&Window) -> bool) -> bool {
    let scene = &mut guest.scene;
    if pick(&scene.win) {
        return true;
    }
    let Some(at) = scene.behind.iter().position(pick) else { return false };
    mem::swap(&mut scene.win, &mut scene.behind[at]);
    true
}

/// Window manager window `id`.
pub fn select_window(guest: &mut Guest, id: u32) -> bool {
    select(guest, |w| w.window == Some(id))
}

/// The window that has the keyboard, as the window manager last said.
pub fn select_focused(guest: &mut Guest) -> bool {
    select(guest, |w| w.focused)
}

/// The window `surface` is shown on: its toplevel's own, or the one a
/// subsurface or a popup of it is laid on, however deep.
pub fn select_for(guest: &mut Guest, surface: u32) -> bool {
    let root = root(&guest.scene, surface);
    select(guest, |w| w.shown.is_some_and(|s| s.surface == root))
}

/// The window xdg-shell object `id` belongs to, an xdg_surface, a toplevel
/// or a popup, brought to `Scene.win`; false when it is on none shown. A
/// request names its own window: only a commit or input selected one, so a
/// maximize, a minimize, a move or a popup asked on one window after another
/// window committed went to that other window, or was dropped.
pub fn select_role(guest: &mut Guest, id: u32) -> bool {
    let scene = &guest.scene;
    let own = scene.surfaces.iter().find(|s| s.xdg == Some(id) || s.toplevel == Some(id));
    let popup = || scene.popups.list.iter().find(|p| p.id == id).map(|p| p.surface);
    match own.map(|s| s.id).or_else(popup) {
        Some(surface) => select_for(guest, surface),
        None => false,
    }
}

/// Whether `surface` is shown on `Scene.win`.
pub fn on_win(scene: &Scene, surface: u32) -> bool {
    let root = root(scene, surface);
    scene.win.shown.is_some_and(|s| s.surface == root)
}

fn root(scene: &Scene, surface: u32) -> u32 {
    let mut at = surface;
    for _ in 0..super::super::subsurface::order::MAX_DEPTH {
        let up = match scene.subs.of(at) {
            Some(sub) => sub.parent,
            None => match scene.popups.list.iter().find(|p| p.surface == at) {
                Some(p) => p.parent,
                None => return at,
            },
        };
        at = up;
    }
    at
}

/// Every window manager window the guest has.
pub fn windows(scene: &Scene) -> Vec<u32> {
    core::iter::once(&scene.win).chain(scene.behind.iter()).filter_map(|w| w.window).collect()
}
