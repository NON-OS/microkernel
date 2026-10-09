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

use super::object::Object;
use super::ops::ev;
use super::out::Event;
use super::args::Args;

/// The size a toplevel is told to be when nothing has asked otherwise.
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn get_xdg_surface(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(surface)) = (args.u32(), args.u32()) else {
        return;
    };
    if !guest.objects.put(id, Object::XdgSurface) {
        return;
    }
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) {
        s.xdg = Some(id);
    }
}

pub fn get_toplevel(guest: &mut Guest, xdg: u32, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if !guest.objects.put(id, Object::XdgToplevel) {
        return;
    }
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.xdg == Some(xdg)) {
        s.toplevel = Some(id);
        // A surface shown before it was made a toplevel goes with this one.
        let surface = s.id;
        if let Some(shown) = guest.scene.shown.as_mut().filter(|w| w.surface == surface) {
            shown.toplevel = Some(id);
        }
    }
    let serial = guest.scene.next_serial();
    // An empty states array, which is a length of zero.
    Event::new(id, ev::TOPLEVEL_CONFIGURE)
        .u32(WIDTH)
        .u32(HEIGHT)
        .u32(0)
        .send(&mut guest.display.to_client);
    Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(serial).send(&mut guest.display.to_client);
}

pub fn ack_configure(guest: &mut Guest, xdg: u32, args: &mut Args<'_>) {
    let Some(serial) = args.u32() else { return };
    let owner = guest.scene.surfaces.iter_mut().find(|s| s.xdg == Some(xdg));
    if let Some(s) = owner {
        s.configured = true;
    }
    // A new size or state is taken with the next buffer (fit.rs).
    guest.scene.fit.ack(serial);
}
