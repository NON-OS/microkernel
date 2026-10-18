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

//! xdg_popup.grab. A popup takes its grab only as rules.rs allows: with the
//! serial of the latest press this client was sent, on the topmost
//! grabbing popup or the shown toplevel. Then it is configured and shown as
//! the topmost layer of its window (layer.rs). Refused, it ends at once.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::ev;
use super::super::ops::shell::POPUP_CONFIGURE;
use super::super::out::Event;
use super::ends::end;
use super::made::origin;

/// grab on popup `id`, with a seat and a serial.
pub fn grab(guest: &mut Guest, id: u32, args: &mut Args<'_>) {
    let (Some(_seat), Some(serial)) = (args.u32(), args.u32()) else { return };
    let root = guest.scene.shown.map(|s| s.surface);
    if !guest.scene.popups.may_grab(id, serial, root) {
        return end(guest, id);
    }
    let list = &mut guest.scene.popups.list;
    let Some(at) = list.iter().position(|p| p.id == id) else { return };
    let mut p = list.remove(at);
    p.grabbed = true;
    let (surface, parent, rect) = (p.surface, p.parent, p.rect);
    // Above every grabbing popup, below those still waiting for one.
    let to = list.iter().rposition(|q| q.grabbed).map_or(0, |t| t + 1);
    list.insert(to, p);
    super::layer::lay(&mut guest.scene, id, surface, (rect.0, rect.1));
    // The protocol gives the place on the parent's surface.
    let (ox, oy) = origin(&guest.scene, parent).unwrap_or((0, 0));
    let (x, y) = (rect.0.wrapping_sub(ox), rect.1.wrapping_sub(oy));
    let xdg = guest.scene.surfaces.iter().find(|s| s.id == surface).and_then(|s| s.xdg);
    let serial = guest.scene.next_serial();
    let to_client = &mut guest.display.to_client;
    let words = [x as u32, y as u32, rect.2 as u32, rect.3 as u32];
    Event::new(id, POPUP_CONFIGURE).words(&words).send(to_client);
    if let Some(xdg) = xdg {
        Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(serial).send(to_client);
    }
}
