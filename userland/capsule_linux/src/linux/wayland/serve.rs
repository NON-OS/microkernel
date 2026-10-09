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


//! Draining the client's requests and answering what is understood.

use core::mem;

use crate::linux::guest::Guest;

use super::object::Object;
use super::ops::req;
use super::route::route;
use super::unserved::unserved;
use super::args::Args;
use super::wire::{malformed, walk, Msg};

/// Serve what has arrived, while the client is reading what it is told: a
/// client that is not has its requests wait (`unix::Conn::backlogged`), and
/// its reads come back here to go on.
///
/// The requests are walked where they lie and the ones served cut from the
/// queue once (`wire::walk`). No handler writes to the queue, so it is held
/// apart from the guest while they run.
pub fn serve(guest: &mut Guest) {
    let mut queue = mem::take(&mut guest.display.to_server);
    let served = match guest.display.backlogged() {
        true => 0,
        false => walk(&queue, |msg| {
            one(guest, msg);
            !guest.display.backlogged()
        }),
    };
    queue.drain(..served);
    guest.display.to_server = queue;
    /*
     * A header that claims less than a header can never complete, so the
     * bytes behind it would wait, and pile up behind every later write, for
     * good. A Wayland server ends such a client; here what it sent is
     * dropped, and said.
     */
    if malformed(&guest.display.to_server) {
        guest.display.to_server.clear();
        crate::linux::say::say(b"[WAYLAND] malformed request header: client stream dropped\n");
    }
}

/// One message, already walked past, so a handler that refuses it cannot
/// have it served again.
fn one(guest: &mut Guest, msg: Msg<'_>) {
    let object = guest.objects.get(msg.object);
    let mut args = Args::new(msg.args);
    if !route(guest, object, msg.object, msg.opcode, &mut args) {
        unserved(object, msg.opcode);
    }
}

/// Requests that only remove something.
pub fn is_destructor(object: Option<Object>, opcode: u16) -> bool {
    matches!(
        (object, opcode),
        (Some(Object::Buffer), req::BUFFER_DESTROY)
            | (Some(Object::ShmPool), req::POOL_DESTROY)
            | (Some(Object::Surface), req::SURFACE_DESTROY)
    )
}
