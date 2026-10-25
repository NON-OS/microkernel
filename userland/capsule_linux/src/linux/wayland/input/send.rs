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

//! A pointer motion, written to the client; a button is input_button.rs.

use crate::linux::guest::Guest;

use super::super::out::Event;
use super::enter::pointer_on;

/// wl_pointer: motion 2, frame 5.
const POINTER_MOTION: u16 = 2;
const POINTER_FRAME: u16 = 5;

/// Wayland carries a surface coordinate as 24.8 fixed point.
fn fixed(value: i32) -> u32 {
    (value << 8) as u32
}

pub fn motion(guest: &mut Guest, x: i32, y: i32) {
    let Some(id) = guest.scene.pointer else { return };
    let Some((x, y)) = pointer_on(guest, id, x, y) else { return };
    let time = time_ms();
    Event::new(id, POINTER_MOTION)
        .u32(time)
        .u32(fixed(x))
        .u32(fixed(y))
        .send(&mut guest.display.to_client);
    frame(guest, id);
}

/// wl_pointer.frame came with version 5; an older pointer takes each event
/// as it comes.
pub fn frame(guest: &mut Guest, id: u32) {
    if guest.objects.at_least(id, 5) {
        Event::new(id, POINTER_FRAME).send(&mut guest.display.to_client);
    }
}

pub fn time_ms() -> u32 {
    crate::linux::call::family_ms() as u32
}
