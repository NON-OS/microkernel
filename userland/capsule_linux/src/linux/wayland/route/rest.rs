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
//! makes is always made. Popups and their positioners are popup/'s. Moving,
//! resizing and minimizing are the window manager's, which the person
//! drives; a parent, size limits and a window menu change nothing either.
//! The seat's requests are route_seat.rs. False means nothing served it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::shell::*;
use super::super::popup;

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
        (Some(XdgSurface), SURFACE_GET_POPUP) => popup::made::get_popup(guest, id, args),
        (Some(Positioner), POSITIONER_SET_FIRST..=POSITIONER_SET_LAST) => {
            popup::pos::position(guest, id, op, args)
        }
        (Some(Popup), POPUP_GRAB) => popup::grab::grab(guest, id, args),
        // A window minimized ends its popups' grabs, as a focus-out will.
        (Some(XdgToplevel), TOPLEVEL_SET_MINIMIZED) => popup::ends::end_all(guest),
        (
            Some(XdgToplevel),
            TOPLEVEL_SET_PARENT
            | TOPLEVEL_SHOW_WINDOW_MENU
            | TOPLEVEL_MOVE
            | TOPLEVEL_RESIZE
            | TOPLEVEL_SET_MAX_SIZE
            | TOPLEVEL_SET_MIN_SIZE,
        ) => {}
        (Some(Positioner), POSITIONER_DESTROY) => {
            popup::pos::forget(guest, id);
            super::super::forget::forget(guest, object, id)
        }
        (Some(Popup), POPUP_DESTROY) => {
            popup::gone::destroyed(guest, id);
            super::super::forget::forget(guest, object, id)
        }
        (Some(XdgBase), BASE_DESTROY) => super::super::forget::forget(guest, object, id),
        _ => return super::seat::route(guest, object, id, op, args),
    }
    true
}

pub(in crate::linux::wayland) fn make(guest: &mut Guest, args: &mut Args<'_>, what: Object) {
    if let Some(id) = args.u32() {
        let _ = guest.objects.put(id, what);
    }
}
