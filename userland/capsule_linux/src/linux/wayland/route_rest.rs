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

//! The rest of xdg-shell, answered as this server can; an object a request
//! makes is always made. A popup is dismissed at once (popup_done): a menu
//! a guest opens is not shown, so a positioner changes nothing. Moving,
//! resizing and minimizing are the window manager's, which the person
//! drives; a parent, size limits and a window menu change nothing either.
//! The seat's requests are route_seat.rs. False means nothing served it.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::shell::*;
use super::out::Event;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    use Object::*;
    match (object, op) {
        (Some(XdgBase), BASE_CREATE_POSITIONER) => make(guest, args, Positioner),
        (Some(XdgSurface), SURFACE_GET_POPUP) => popup(guest, args),
        (Some(Positioner), POSITIONER_SET_FIRST..=POSITIONER_SET_LAST) => {}
        (Some(Popup), POPUP_GRAB) => {}
        (
            Some(XdgToplevel),
            TOPLEVEL_SET_PARENT
            | TOPLEVEL_SHOW_WINDOW_MENU
            | TOPLEVEL_MOVE
            | TOPLEVEL_RESIZE
            | TOPLEVEL_SET_MAX_SIZE
            | TOPLEVEL_SET_MIN_SIZE
            | TOPLEVEL_SET_MINIMIZED,
        ) => {}
        (Some(XdgBase), BASE_DESTROY)
        | (Some(Positioner), POSITIONER_DESTROY)
        | (Some(Popup), POPUP_DESTROY) => super::forget::forget(guest, object, id),
        _ => return super::route_seat::route(guest, object, id, op, args),
    }
    true
}

pub(super) fn make(guest: &mut Guest, args: &mut Args<'_>, what: Object) {
    if let Some(id) = args.u32() {
        let _ = guest.objects.put(id, what);
    }
}

fn popup(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if guest.objects.put(id, Object::Popup) {
        Event::new(id, POPUP_DONE).send(&mut guest.display.to_client);
    }
}
