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

//! A popup's configure: where it is on its parent's surface and how large,
//! then its xdg_surface's configure with the serial the client acks.

use crate::linux::guest::Guest;

use super::super::ops::ev;
use super::super::ops::shell::POPUP_CONFIGURE;
use super::super::out::Event;
use super::made::origin;

/// Popup `id` told its place, which the protocol gives on the parent's
/// surface; the server keeps it on the toplevel's.
pub fn configure(guest: &mut Guest, id: u32) {
    let Some(p) = guest.scene.popups.list.iter().find(|p| p.id == id) else { return };
    let (surface, parent, rect) = (p.surface, p.parent, p.rect);
    let (ox, oy) = origin(&guest.scene, parent).unwrap_or((0, 0));
    let (x, y) = (rect.0.wrapping_sub(ox), rect.1.wrapping_sub(oy));
    let xdg = guest.scene.surfaces.iter().find(|s| s.id == surface).and_then(|s| s.xdg);
    let serial = guest.scene.next_serial();
    let to_client = &mut guest.display.to_client;
    // Four ints, x, y, width and height: not an array.
    let ev = Event::new(id, POPUP_CONFIGURE).u32(x as u32).u32(y as u32);
    ev.u32(rect.2 as u32).u32(rect.3 as u32).send(to_client);
    if let Some(xdg) = xdg {
        Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(serial).send(to_client);
    }
}
