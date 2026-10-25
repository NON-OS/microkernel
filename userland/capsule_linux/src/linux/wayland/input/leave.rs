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

//! Leaving the surface a pointer or keyboard was last told it is on.

use crate::linux::guest::Guest;

use super::super::focus_step::{step, Step};
use super::super::object::Object;
use super::super::out::Event;

pub const POINTER_LEAVE: u16 = 1;
pub const KEYBOARD_LEAVE: u16 = 2;

/// Leaves the surface last entered (`was`) for `target`, as focus_step.rs
/// says, and gives the one to enter; or Err with whether the event goes on
/// (it is on `target` already) or is dropped (there is none).
pub fn move_from(
    guest: &mut Guest,
    device: u32,
    leave_op: u16,
    (was, target): (Option<u32>, Option<u32>),
) -> Result<u32, bool> {
    match step(was, target, |id| guest.objects.get(id) == Some(Object::Surface)) {
        Step::Drop => Err(false),
        Step::Stay => Err(true),
        Step::Move { leave, on } => {
            if let Some(old) = leave {
                let serial = guest.scene.next_serial();
                Event::new(device, leave_op)
                    .u32(serial)
                    .u32(old)
                    .send(&mut guest.display.to_client);
            }
            Ok(on)
        }
    }
}
