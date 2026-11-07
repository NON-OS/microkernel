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

//! The xdg-shell requests: a surface given a window's role, and the window.
//! False means nothing served it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::fit::Mode;
use super::super::object::Object;
use super::super::ops::req;
use super::super::{toplevel, xdg};

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    opcode: u16,
    args: &mut Args<'_>,
) -> bool {
    if names_its_window(object, opcode) {
        super::super::window_front::select_role(guest, id);
    }
    match (object, opcode) {
        (Some(Object::XdgBase), req::XDG_BASE_GET_SURFACE) => {
            xdg::get_xdg_surface(guest, id, args)
        }
        (Some(Object::XdgBase), req::XDG_BASE_PONG) => {}
        (Some(Object::XdgSurface), req::XDG_SURFACE_GET_TOPLEVEL) => {
            xdg::get_toplevel(guest, id, args)
        }
        (Some(Object::XdgSurface), req::XDG_SURFACE_ACK_CONFIGURE) => {
            xdg::ack_configure(guest, id, args)
        }
        (Some(Object::XdgSurface), req::XDG_SURFACE_SET_GEOMETRY) => {
            super::super::toplevel::geometry(guest, id, args)
        }
        (Some(Object::XdgSurface), req::XDG_SURFACE_DESTROY) => {
            xdg::destroyed(guest, id);
            super::super::forget::forget(guest, object, id)
        }
        (Some(Object::XdgToplevel), req::TOPLEVEL_SET_TITLE) => {
            let title = args.string().unwrap_or(b"");
            toplevel::title(guest, id, title)
        }
        (Some(Object::XdgToplevel), req::TOPLEVEL_SET_APP_ID) => {}
        (Some(Object::XdgToplevel), req::TOPLEVEL_DESTROY) => toplevel::destroy(guest, id),
        (Some(Object::XdgToplevel), req::TOPLEVEL_SET_MAXIMIZED) => {
            toplevel::set(guest, id, Mode::Maximized, args)
        }
        (Some(Object::XdgToplevel), req::TOPLEVEL_SET_FULLSCREEN) => {
            toplevel::set(guest, id, Mode::FullScreen, args)
        }
        (Some(Object::XdgToplevel), req::TOPLEVEL_UNSET_MAXIMIZED) => {
            toplevel::unset(guest, id, Mode::Maximized)
        }
        (Some(Object::XdgToplevel), req::TOPLEVEL_UNSET_FULLSCREEN) => {
            toplevel::unset(guest, id, Mode::FullScreen)
        }
        _ => return super::super::data_device::route(guest, object, id, opcode, args),
    }
    true
}

/// A request that acts on the window its object is on: an ack lands on that
/// window's configure, a mode on that window.
fn names_its_window(object: Option<Object>, opcode: u16) -> bool {
    matches!(
        (object, opcode),
        (Some(Object::XdgSurface), req::XDG_SURFACE_ACK_CONFIGURE)
            | (
                Some(Object::XdgToplevel),
                req::TOPLEVEL_SET_MAXIMIZED
                    | req::TOPLEVEL_SET_FULLSCREEN
                    | req::TOPLEVEL_UNSET_MAXIMIZED
                    | req::TOPLEVEL_UNSET_FULLSCREEN
            )
    )
}
