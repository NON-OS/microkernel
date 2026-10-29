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

//! org_kde_kwin_server_decoration_manager, version 1: how GTK 3 asks who
//! draws a window's frame. It binds no xdg-decoration and reads GTK_CSD
//! nowhere on Wayland; without this manager, or with a default other than
//! Server, it draws its own title bar inside the NØNOS frame. It is told
//! Server as it binds, and a window's mode is the switch xdg-decoration
//! sets too (decoration.rs, present/frame/).

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::out::Event;

#[path = "kde_mode.rs"]
pub mod mode;

const MANAGER_CREATE: u16 = 0;
const MANAGER_DEFAULT_MODE: u16 = 0;
const DECORATION_RELEASE: u16 = 0;
const DECORATION_REQUEST_MODE: u16 = 1;
const DECORATION_MODE: u16 = 0;

/// The default mode, said as the manager is bound.
pub fn bound(guest: &mut Guest, manager: u32) {
    let ev = Event::new(manager, MANAGER_DEFAULT_MODE).u32(mode::SERVER);
    ev.send(&mut guest.display.to_client);
}

pub fn route(g: &mut Guest, object: Option<Object>, id: u32, op: u16, a: &mut Args<'_>) -> bool {
    match (object, op) {
        (Some(Object::KdeDecorationManager), MANAGER_CREATE) => {
            let (Some(made), Some(_surface)) = (a.u32(), a.u32()) else { return true };
            if g.objects.put(made, Object::KdeDecoration) {
                Event::new(made, DECORATION_MODE).u32(mode::SERVER).send(&mut g.display.to_client);
            }
        }
        (Some(Object::KdeDecoration), DECORATION_REQUEST_MODE) => {
            let (own, told) = mode::answer(a.u32().unwrap_or(mode::NONE));
            g.scene.frame.own_frame = own;
            Event::new(id, DECORATION_MODE).u32(told).send(&mut g.display.to_client);
        }
        (Some(Object::KdeDecoration), DECORATION_RELEASE) => {
            super::super::forget::forget(g, object, id)
        }
        _ => return super::super::route::rest::route(g, object, id, op, a),
    }
    true
}
