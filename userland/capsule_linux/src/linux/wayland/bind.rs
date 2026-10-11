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

//! `wl_registry.bind`. The client names a global, its interface and the
//! version it wants; libwayland's server ends a client that names one it
//! was never offered, or asks past the offered version, and so does this.

use alloc::format;

use crate::linux::guest::Guest;

use super::args::Args;
use super::global::by_name;
use super::object::Object;
use super::out::Event;

const SEAT_CAPABILITIES: u16 = 0;
/// A pointer and a keyboard, which is what the input events carry.
const SEAT_POINTER_AND_KEYBOARD: u32 = 3;

pub fn bind(guest: &mut Guest, registry: u32, args: &mut Args<'_>) {
    let (Some(name), Some(interface), Some(version), Some(id)) =
        (args.u32(), args.string(), args.u32(), args.u32())
    else {
        return;
    };
    let Some(global) = by_name(name) else {
        let message = format!("invalid global {name}");
        return super::display_error::fatal(guest, registry, &message);
    };
    if interface != global.interface || version == 0 || version > global.version {
        let message = format!("invalid bind of global {name} at version {version}");
        return super::display_error::fatal(guest, registry, &message);
    }
    if !guest.objects.bound(id, global.object, version) {
        return;
    }
    match global.object {
        /*
         * A client reads the format list before it asks for a pool, and
         * a seat's capabilities before it asks for a keyboard. Both are
         * sent as the bind is made, as are the output's details.
         */
        Object::Shm => super::shm::formats(guest, id),
        Object::Seat => seat_caps(guest, id),
        Object::Output => {
            super::output::announce(guest, id);
            super::output_enter::bound(guest, id);
        }
        _ => {}
    }
}

fn seat_caps(guest: &mut Guest, id: u32) {
    Event::new(id, SEAT_CAPABILITIES)
        .u32(SEAT_POINTER_AND_KEYBOARD)
        .send(&mut guest.display.to_client);
}
