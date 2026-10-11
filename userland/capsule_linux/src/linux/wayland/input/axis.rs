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

//! A wheel step, written to the client as wl_pointer v5 frames it: where it
//! came from, the notches, the distance, then the frame.

use crate::linux::guest::Guest;

use super::super::out::Event;
use super::super::pointer_codes::wheel_step;
use super::enter::pointer_on;

const POINTER_AXIS: u16 = 4;
const POINTER_FRAME: u16 = 5;
const POINTER_AXIS_SOURCE: u16 = 6;
const POINTER_AXIS_DISCRETE: u16 = 8;
const SOURCE_WHEEL: u32 = 0;
const VERTICAL: u32 = 0;
const HORIZONTAL: u32 = 1;

pub fn wheel(guest: &mut Guest, dx: i32, dy: i32, x: i32, y: i32) {
    let Some(id) = guest.scene.pointer else { return };
    let steps = [(VERTICAL, wheel_step(dy)), (HORIZONTAL, wheel_step(dx))];
    if steps.iter().all(|(_, s)| s.is_none()) {
        return;
    }
    if pointer_on(guest, id, x, y).is_none() {
        return;
    }
    let time = crate::linux::call::family_ms() as u32;
    // The source, the notches and the frame came with wl_pointer 5.
    let v5 = guest.objects.at_least(id, 5);
    let to_client = &mut guest.display.to_client;
    if v5 {
        Event::new(id, POINTER_AXIS_SOURCE).u32(SOURCE_WHEEL).send(to_client);
    }
    for (axis, step) in steps {
        let Some((notches, distance)) = step else { continue };
        if v5 {
            Event::new(id, POINTER_AXIS_DISCRETE).u32(axis).u32(notches as u32).send(to_client);
        }
        Event::new(id, POINTER_AXIS).u32(time).u32(axis).u32(distance).send(to_client);
    }
    if v5 {
        Event::new(id, POINTER_FRAME).send(to_client);
    }
}
