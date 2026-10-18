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

use crate::linux::guest::Guest;

use super::args::Args;
use super::route::all::route;
use super::unserved::unserved;
use super::wire::{malformed, walk, Msg};

/// Serve what has arrived while the client reads what it is told; one that
/// does not has its requests wait (`unix::Conn::backlogged`). They are walked
/// where they lie and the served cut once (`wire::walk`); no handler writes
/// the queue, so it is held apart from the guest while they run.
pub fn serve(guest: &mut Guest) {
    let mut queue = core::mem::take(&mut guest.display.to_server);
    let mut ended = guest.objects.failed();
    let served = match guest.display.backlogged() || ended {
        true => 0,
        false => walk(&queue, |msg| {
            let fine = one(guest, msg);
            ended |= !fine;
            fine && !guest.display.backlogged()
        }),
    };
    queue.drain(..served);
    // After a fatal error nothing more from this client is served, ever.
    if ended {
        queue.clear();
    }
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
/// have it served again. False when it ends the client: it names an object
/// the client never made, or was fatal itself (display_error.rs).
fn one(guest: &mut Guest, msg: Msg<'_>) -> bool {
    let Some(object) = guest.objects.get(msg.object) else {
        super::display_error::invalid_object(guest, msg.object);
        return false;
    };
    guest.objects.serving(msg.object);
    let mut args = Args::new(msg.args);
    if !route(guest, Some(object), msg.object, msg.opcode, &mut args) {
        unserved(Some(object), msg.opcode);
    }
    !guest.objects.failed()
}
