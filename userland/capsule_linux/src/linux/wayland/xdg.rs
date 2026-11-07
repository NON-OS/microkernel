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

//! `xdg_wm_base` and the two objects a toplevel window is made of.

use crate::linux::guest::Guest;

use super::args::Args;
use super::display_error::protocol;
use super::object::Object;

/// xdg_wm_base.error.role: the wl_surface has another role.
const ROLE: u32 = 0;
/// xdg_surface.error.already_constructed: it has a role object already.
const ALREADY_CONSTRUCTED: u32 = 2;
/// xdg_surface.error.defunct_role_object: destroyed before its role object.
const DEFUNCT_ROLE_OBJECT: u32 = 6;

pub fn get_xdg_surface(guest: &mut Guest, base: u32, args: &mut Args<'_>) {
    let (Some(id), Some(surface)) = (args.u32(), args.u32()) else {
        return;
    };
    let taken = guest.scene.surfaces.iter().any(|s| s.id == surface && s.xdg.is_some());
    if taken {
        return protocol(guest, base, ROLE, "wl_surface has an xdg_surface already");
    }
    if !guest.objects.put(id, Object::XdgSurface) {
        return;
    }
    let not_placed = guest.scene.subs.of(surface).is_none();
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface && not_placed) {
        s.xdg = Some(id);
    }
}

pub fn get_toplevel(guest: &mut Guest, xdg: u32, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    let scene = &guest.scene;
    let surface = scene.surfaces.iter().find(|s| s.xdg == Some(xdg));
    let popup = |s: u32| scene.popups.list.iter().any(|p| p.surface == s);
    if surface.is_some_and(|s| s.toplevel.is_some() || popup(s.id)) {
        return protocol(guest, xdg, ALREADY_CONSTRUCTED, "xdg_surface has a role object already");
    }
    if !guest.objects.put(id, Object::XdgToplevel) {
        return;
    }
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.xdg == Some(xdg)) {
        s.toplevel = Some(id);
        // Its first configure answers its initial commit (toplevel/first.rs).
        s.initial = true;
        // A surface shown before it was made a toplevel goes with this one.
        let surface = s.id;
        if let Some(shown) = guest.scene.win.shown.as_mut().filter(|w| w.surface == surface) {
            shown.toplevel = Some(id);
        }
    }
}

pub fn ack_configure(guest: &mut Guest, xdg: u32, args: &mut Args<'_>) {
    let Some(serial) = args.u32() else { return };
    let owner = guest.scene.surfaces.iter_mut().find(|s| s.xdg == Some(xdg));
    if let Some(s) = owner {
        s.configured = true;
    }
    // A new size or state is taken with the next buffer (fit.rs).
    guest.scene.win.fit.ack(serial);
}

/// xdg_surface `id` destroyed: its surface is no xdg_surface's any more, and
/// one destroyed before its toplevel is the client's error.
pub fn destroyed(guest: &mut Guest, id: u32) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.xdg == Some(id)) else { return };
    if s.toplevel.is_some() {
        return protocol(guest, id, DEFUNCT_ROLE_OBJECT, "xdg_surface destroyed before its toplevel");
    }
    s.xdg = None;
}
