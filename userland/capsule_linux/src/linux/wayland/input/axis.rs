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

//! A wheel step, written to the client as wl_pointer frames it: where it
//! came from, the notches, the distance, then the frame. A pointer made at
//! version 8 or later is told the notches as axis_value120 (a notch is 120)
//! in place of axis_discrete, which a client with a high-resolution wheel
//! reads; one at 9 or later is told the scroll is not inverted. A wheel
//! here moves in whole notches, so the two say the same.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::out::Event;
use super::super::pointer_codes::wheel_step;
use super::enter::pointer_on;

const POINTER_AXIS: u16 = 4;
const POINTER_FRAME: u16 = 5;
const POINTER_AXIS_SOURCE: u16 = 6;
const POINTER_AXIS_DISCRETE: u16 = 8;
const POINTER_AXIS_VALUE120: u16 = 9;
const POINTER_AXIS_RELATIVE_DIRECTION: u16 = 10;
/// The wheel's motion and the content's go the same way.
const IDENTICAL: u32 = 0;
const SOURCE_WHEEL: u32 = 0;
const VERTICAL: u32 = 0;
const HORIZONTAL: u32 = 1;

pub fn wheel(guest: &mut Guest, dx: i32, dy: i32, x: i32, y: i32) {
    let steps = [(VERTICAL, wheel_step(dy)), (HORIZONTAL, wheel_step(dx))];
    if steps.iter().all(|(_, s)| s.is_none()) || pointer_on(guest, x, y).is_none() {
        return;
    }
    let time = crate::linux::call::family_ms() as u32;
    for id in super::devices::pointers(guest) {
        let v = guest.objects.version(id);
        notch(&mut guest.display.to_client, (id, v), time, &steps);
    }
}

/// One pointer, made at version `v`, told the step on each axis that moved.
fn notch(to: &mut Vec<u8>, (id, v): (u32, u32), time: u32, steps: &[(u32, Option<(i32, u32)>)]) {
    // The source, the notches and the frame came with wl_pointer 5.
    if v >= 5 {
        Event::new(id, POINTER_AXIS_SOURCE).u32(SOURCE_WHEEL).send(to);
    }
    for &(axis, step) in steps {
        let Some((notches, distance)) = step else { continue };
        match v {
            8.. => {
                let v120 = notches.saturating_mul(120) as u32;
                Event::new(id, POINTER_AXIS_VALUE120).u32(axis).u32(v120).send(to);
            }
            5..=7 => {
                let ev = Event::new(id, POINTER_AXIS_DISCRETE).u32(axis).u32(notches as u32);
                ev.send(to);
            }
            _ => {}
        }
        if v >= 9 {
            Event::new(id, POINTER_AXIS_RELATIVE_DIRECTION).u32(axis).u32(IDENTICAL).send(to);
        }
        Event::new(id, POINTER_AXIS).u32(time).u32(axis).u32(distance).send(to);
    }
    if v >= 5 {
        Event::new(id, POINTER_FRAME).send(to);
    }
}
