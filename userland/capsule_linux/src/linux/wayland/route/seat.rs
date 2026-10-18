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

//! The seat's requests beyond a pointer and a keyboard. Touch is never
//! sent, as the seat's capabilities say, so a touch object only waits for
//! its release. The cursor is the compositor's own, so set_cursor's surface
//! is taken and never shown (commit.rs). A released pointer or keyboard is
//! sent nothing more (forget.rs). False means nothing served it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::seat::*;
use super::rest::make;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    use Object::*;
    match (object, op) {
        (Some(Seat), SEAT_GET_TOUCH) => make(guest, args, Touch),
        (Some(Pointer), POINTER_SET_CURSOR) => {}
        (Some(Seat), SEAT_RELEASE)
        | (Some(Pointer), POINTER_RELEASE)
        | (Some(Keyboard), KEYBOARD_RELEASE)
        | (Some(Touch), TOUCH_RELEASE) => super::super::forget::forget(guest, object, id),
        _ => return super::output::route(guest, object, id, op, args),
    }
    true
}
