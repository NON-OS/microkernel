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

//! The configure a toplevel is told a new size and state with: the
//! toplevel's configure (width, height, the states array), then its
//! xdg_surface's configure with the serial the guest acks. The states are
//! the mode's, and `activated` while the window has the keyboard: Qt takes
//! a window's activation from that state alone, and GTK draws a window
//! without it as a background one. A toplevel made at version 4 or later is
//! first told the bounds it can fill (configure_bounds), which a client
//! picking its own size keeps within. Pure, so the host proofs read the bytes
//! back.

use alloc::vec::Vec;

use super::fit::Configure;
use super::ops::ev;
use super::out::Event;

/// xdg_toplevel.state activated.
pub const STATE_ACTIVATED: u32 = 4;
/// xdg_toplevel.configure_bounds, since version 4.
pub const TOPLEVEL_CONFIGURE_BOUNDS: u16 = 2;

/// `bounds` is said only to a toplevel that can read it: None below 4.
pub fn configure(
    toplevel: u32,
    xdg: u32,
    told: &Configure,
    (activated, bounds): (bool, Option<(u32, u32)>),
    to_client: &mut Vec<u8>,
) {
    if let Some((w, h)) = bounds {
        Event::new(toplevel, TOPLEVEL_CONFIGURE_BOUNDS).u32(w).u32(h).send(to_client);
    }
    let (width, height) = told.size();
    let mut states = told.mode.states().to_vec();
    if activated {
        states.push(STATE_ACTIVATED);
    }
    Event::new(toplevel, ev::TOPLEVEL_CONFIGURE)
        .u32(width)
        .u32(height)
        .words(&states)
        .send(to_client);
    Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(told.serial).send(to_client);
}
