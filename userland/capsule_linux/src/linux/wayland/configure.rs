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
//! xdg_surface's configure with the serial the guest acks. Pure, so the host
//! proofs read the bytes back.

use alloc::vec::Vec;

use super::fit::Configure;
use super::ops::ev;
use super::out::Event;

pub fn configure(toplevel: u32, xdg: u32, told: &Configure, to_client: &mut Vec<u8>) {
    let (width, height) = told.size();
    Event::new(toplevel, ev::TOPLEVEL_CONFIGURE)
        .u32(width)
        .u32(height)
        .words(told.mode.states())
        .send(to_client);
    Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(told.serial).send(to_client);
}
