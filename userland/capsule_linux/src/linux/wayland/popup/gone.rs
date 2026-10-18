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

//! Popups ended by what their client does: a popup destroyed, its parent
//! destroyed, or a commit on a popup that holds no grab, as a tooltip's
//! is. A tooltip is not shown in v1: it gets popup_done at its first
//! commit (rules.rs).

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::object::Object;
use super::ends::{end, finish};

/// Popup `id`, destroyed by its client: those above it end; it goes
/// without a popup_done, its object already gone.
pub fn destroyed(guest: &mut Guest, id: u32) {
    let gone = guest.scene.popups.ending(id);
    finish(guest, &gone, Some(id));
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

/// A commit on `surface`: if it is a popup's that holds no grab, it ends.
pub fn unheld_commit(guest: &mut Guest, surface: u32) {
    let popups = &guest.scene.popups.list;
    if let Some(p) = popups.iter().find(|p| p.surface == surface && !p.grabbed) {
        let id = p.id;
        end(guest, id);
    }
}
