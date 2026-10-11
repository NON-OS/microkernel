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

//! Linux button codes in the numbers the input router uses: 1 left,
//! 2 right, 3 middle, then the side buttons, as the USB driver numbers its
//! report bits.

use crate::constants::evdev::{
    BTN_EXTRA, BTN_GEAR_DOWN, BTN_GEAR_UP, BTN_LEFT, BTN_MIDDLE, BTN_RIGHT, BTN_SIDE,
};

pub const fn button_number(code: u16) -> Option<u32> {
    match code {
        BTN_LEFT => Some(1),
        BTN_RIGHT => Some(2),
        BTN_MIDDLE => Some(3),
        BTN_SIDE => Some(4),
        BTN_EXTRA => Some(5),
        _ => None,
    }
}

/// A wheel step from QEMU's gear buttons, on their press only: up (away
/// from the user) is +1, as REL_WHEEL counts it.
pub const fn gear_step(code: u16, value: i32) -> Option<i32> {
    match (code, value) {
        (BTN_GEAR_UP, 1) => Some(1),
        (BTN_GEAR_DOWN, 1) => Some(-1),
        (BTN_GEAR_UP | BTN_GEAR_DOWN, _) => Some(0),
        _ => None,
    }
}
