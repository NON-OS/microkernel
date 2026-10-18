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

//! A packet's four words read into what the input ring takes.

use crate::constants::{BUTTON_LEFT, BUTTON_MIDDLE, BUTTON_RIGHT, RELATIVE_PACKET};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Motion {
    /// A position on the host's 0 to 0xFFFF grid, as the host gave it.
    Abs(i32, i32),
    /// Motion, with Y down, while the host has the pointer grabbed.
    Rel(i32, i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Packet {
    /// Left, right and middle in bits 0, 1 and 2, as the PS/2 mouse has them.
    pub buttons: u8,
    pub motion: Motion,
    /// Notches away from the user, positive.
    pub wheel: i32,
}

pub fn parse(w: [u32; 4]) -> Packet {
    let mut buttons = 0;
    for (i, bit) in [BUTTON_LEFT, BUTTON_RIGHT, BUTTON_MIDDLE].into_iter().enumerate() {
        if w[0] & bit != 0 {
            buttons |= 1 << i;
        }
    }
    let motion = if w[0] & RELATIVE_PACKET != 0 {
        // The host counts Y up for motion; the ring counts it down.
        Motion::Rel(w[1] as i32, (w[2] as i32).wrapping_neg())
    } else {
        Motion::Abs(w[1] as i32, w[2] as i32)
    };
    // The wheel byte counts a notch toward the user as +1, as PS/2 does.
    Packet { buttons, motion, wheel: -i32::from(w[3] as u8 as i8) }
}
