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

//! Popups ended by what their client does: a popup destroyed or unmapped,
//! or its parent destroyed.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::object::Object;
use super::super::scene::Scene;
use super::ends::{end, finish};

/// Popup `id`, destroyed by its client: those above it end; it goes
/// without a popup_done, its object already gone.
pub fn destroyed(guest: &mut Guest, id: u32) {
    let gone = guest.scene.popups.ending(id);
    finish(guest, &gone, Some(id));
}

/// The popup on `surface` that is shown, configured at a commit before.
/// Asked before a commit's own first configure (show.rs first), so that
/// commit, the popup's initial one, is never taken for an unmap.
pub fn shown_on(scene: &Scene, surface: u32) -> Option<u32> {
    scene.popups.list.iter().find(|p| p.surface == surface && p.shown).map(|p| p.id)
}

/// Popup `id`, unmapped by its client: a null buffer attached and committed
/// on its surface, as Qt and GTK hide a menu or a tooltip. It ends with
/// those above it, each told popup_done, as xdg-shell lets a compositor
/// dismiss a popup at any time, so its client makes a new one to show it
/// again; its window is closed with the window manager and its layer taken
/// off the screen. Kept, the window went on taking every press on the place
/// it covered, over whatever was drawn there, and its last pixels stayed.
pub fn unmapped(guest: &mut Guest, id: u32) {
    end(guest, id);
}

/// `id`, a wl_surface or an xdg_surface (`what`), destroyed: each popup
/// whose parent it was ends, and the popup it was, if one.
pub fn parent_gone(guest: &mut Guest, what: Option<Object>, id: u32) {
    let surface = match what {
        Some(Object::Surface) => Some(id),
        Some(Object::XdgSurface) => {
            guest.scene.surfaces.iter().find(|s| s.xdg == Some(id)).map(|s| s.id)
        }
        _ => None,
    };
    let Some(surface) = surface else { return };
    let popups = &guest.scene.popups.list;
    let hit: Vec<u32> = popups
        .iter()
        .filter(|p| p.parent == surface || p.surface == surface)
        .map(|p| p.id)
        .collect();
    for id in hit {
        end(guest, id);
    }
}
