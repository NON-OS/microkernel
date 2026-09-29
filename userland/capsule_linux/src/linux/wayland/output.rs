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

//! `wl_output`: the screen a client asks about before it draws. Clients size
//! their first buffer from the mode and scale, so they are told the display
//! the compositor actually drives, and then `done`.

use nonos_app_skeleton::clients::compositor::display_info;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::out::Event;

const GEOMETRY: u16 = 0;
const MODE: u16 = 1;
const DONE: u16 = 2;
const SCALE: u16 = 3;
const MODE_CURRENT_PREFERRED: u32 = 0x3;
const REFRESH_MHZ: u32 = 60_000;
// What a client is told when the compositor cannot be asked; better a
// plausible screen than none, since a client with no mode draws nothing.
const FALLBACK: (u32, u32) = (1280, 800);

pub fn announce(guest: &mut Guest, id: u32) {
    let (w, h) = lookup_port(b"compositor")
        .and_then(|port| display_info(port, 1).ok())
        .map(|d| (d.width, d.height))
        .unwrap_or(FALLBACK);
    // Physical size at 96 dpi; nothing reports the panel's real size.
    let mm = |px: u32| px.saturating_mul(254) / 960;
    let to = &mut guest.display.to_client;
    Event::new(id, GEOMETRY)
        .u32(0)
        .u32(0)
        .u32(mm(w))
        .u32(mm(h))
        .u32(0)
        .string(b"NONOS")
        .string(b"compositor")
        .u32(0)
        .send(to);
    Event::new(id, MODE).u32(MODE_CURRENT_PREFERRED).u32(w).u32(h).u32(REFRESH_MHZ).send(to);
    Event::new(id, SCALE).u32(1).send(to);
    Event::new(id, DONE).send(to);
}
