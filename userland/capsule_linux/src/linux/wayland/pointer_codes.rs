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

//! A NONOS pointer event as wl_pointer reads it. Pure, so the host proofs
//! hold it.
//!
//! The drivers number buttons from 1: left, right, middle, then the side
//! and extra buttons, as both PS/2 and HID order them. A client reads
//! Linux's BTN_LEFT (0x110) onward. A wheel notch away from the person is
//! +1 on the input ring; Linux, and so Wayland, counts scrolling up as a
//! negative axis, and one notch as 15 units, as libinput does.

/// BTN_LEFT, BTN_RIGHT, BTN_MIDDLE, BTN_SIDE, BTN_EXTRA.
pub fn button_code(button: u32) -> Option<u32> {
    (1..=5).contains(&button).then(|| 0x10F + button)
}

/// One wheel step on an axis: the notches, and the distance in 24.8 fixed
/// point. None when it did not move.
pub fn wheel_step(delta: i32) -> Option<(i32, u32)> {
    let notches = delta.checked_neg().filter(|n| *n != 0)?;
    let units = notches.checked_mul(15)?.checked_mul(256)?;
    Some((notches, units as u32))
}
