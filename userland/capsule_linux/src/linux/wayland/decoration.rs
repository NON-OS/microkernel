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

//! `zxdg_decoration_manager_v1`: who draws a window's frame. The server,
//! unless the client asks to draw its own: a guest's window is framed like
//! every native one (present/frame/). The
//! first answer is a configure sequence, ended by its xdg_surface's
//! configure, whose serial the client acks.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::decoration::*;
use super::ops::ev;
use super::out::Event;

#[path = "kde_decoration.rs"]
pub mod kde;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    match (object, op) {
        (Some(Object::DecorationManager), MANAGER_GET_TOPLEVEL_DECORATION) => get(guest, args),
        /* A client that draws its own frame (libadwaita) is let draw it,
         * and gets no NØNOS frame; any other is framed by the personality
         * (present/frame/). Said again with no new serial; the next
         * xdg_surface configure ends the sequence it joins. */
        (Some(Object::Decoration), DECORATION_SET_MODE | DECORATION_UNSET_MODE) => {
            let own = op == DECORATION_SET_MODE && args.u32() == Some(CLIENT_SIDE);
            own_frame(guest, id, own);
            let mode = if own { CLIENT_SIDE } else { SERVER_SIDE };
            Event::new(id, DECORATION_CONFIGURE).u32(mode).send(&mut guest.display.to_client)
        }
        (Some(Object::DecorationManager), MANAGER_DESTROY)
        | (Some(Object::Decoration), DECORATION_DESTROY) => {
            super::forget::forget(guest, object, id)
        }
        _ => return kde::route(guest, object, id, op, args),
    }
    true
}

/// Made only for a toplevel whose xdg_surface ends the sequence. Before
/// the toplevel's first configure, that configure ends it (toplevel/first.rs):
/// a sequence ended now would have the client draw before it is told a size.
fn get(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(toplevel)) = (args.u32(), args.u32()) else { return };
    let owner = guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel));
    let Some((surface, xdg, initial)) = owner.and_then(|s| Some((s.id, s.xdg?, s.initial))) else {
        return;
    };
    if !guest.objects.put(id, Object::Decoration) {
        return;
    }
    guest.scene.decorations.push((id, surface));
    // Framed by the server until the client asks otherwise, whatever its
    // initial commit decided without this object (undecorated).
    own_frame(guest, id, false);
    let serial = guest.scene.next_serial();
    let to_client = &mut guest.display.to_client;
    Event::new(id, DECORATION_CONFIGURE).u32(SERVER_SIDE).send(to_client);
    if !initial {
        Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(serial).send(to_client);
    }
}

/// A toplevel's initial commit on `surface`, with no decoration made for it
/// in either protocol by a client that bound zxdg_decoration_manager_v1:
/// the client knows it could have asked for a frame and did not, so it
/// draws its own, as the protocol has it ("clients continue to
/// self-decorate as they see fit"). Qt makes no decoration for a frameless
/// window, which Telegram's is: it draws its own title bar, and framed it
/// had two. A client that never bound the manager is framed as before.
pub fn undecorated(guest: &mut Guest, surface: u32, toplevel: u32) {
    // Only an xdg_toplevel can be given a decoration (not a zxdg_shell_v6 one).
    let stable = guest.objects.get(toplevel) == Some(Object::XdgToplevel);
    let knows = stable && guest.objects.all(Object::DecorationManager).next().is_some();
    let made = guest.scene.decorations.iter().any(|&(_, s)| s == surface);
    if knows && !made {
        own_surface_frame(&mut guest.scene, surface, true);
    }
}

/// Whether the toplevel `decoration` was made for draws its own frame, kept
/// with its surface; its window's frame follows at once when it is shown.
pub fn own_frame(guest: &mut Guest, decoration: u32, own: bool) {
    let scene = &mut guest.scene;
    let Some(&(_, surface)) = scene.decorations.iter().find(|d| d.0 == decoration) else { return };
    own_surface_frame(scene, surface, own);
}

/// `surface` draws its own frame or not, and its window's frame follows at
/// once when it is shown.
fn own_surface_frame(scene: &mut super::scene::Scene, surface: u32, own: bool) {
    if let Some(s) = scene.surfaces.iter_mut().find(|s| s.id == surface) {
        s.own_frame = own;
    }
    if scene.win.shown.is_some_and(|w| w.surface == surface) {
        scene.win.frame.own_frame = own;
    }
}
