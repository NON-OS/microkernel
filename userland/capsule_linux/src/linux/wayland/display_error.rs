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

//! `wl_display.error`: a request named an object the client never made, or
//! one already gone, or bound a global wrongly (bind.rs). A Wayland server
//! treats that as fatal, and libwayland ends the client on reading it. It
//! is safe only because every object a request makes is made (route_rest.rs,
//! route_seat.rs): an id this table does not hold is the client's mistake,
//! never one this server left.

use alloc::format;

use crate::linux::guest::Guest;

use super::ops::{ev::DISPLAY_ERROR, ERROR_INVALID_OBJECT};
use super::out::Event;

pub fn invalid_object(guest: &mut Guest, id: u32) {
    let message = format!("object {id} does not exist");
    fatal(guest, id, &message);
}

/// A request to `object` that a Wayland server ends the client for; after
/// it, the client is served nothing more (serve.rs).
pub fn fatal(guest: &mut Guest, object: u32, message: &str) {
    guest.objects.fail();
    Event::new(1, DISPLAY_ERROR)
        .u32(object)
        .u32(ERROR_INVALID_OBJECT)
        .string(message.as_bytes())
        .send(&mut guest.display.to_client);
    crate::linux::say::say(format!("[WAYLAND] {message}: client ended\n").as_bytes());
}
