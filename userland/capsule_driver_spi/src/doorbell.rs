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

//! What OPEN tells the holder, and the doorbell it asks. The holder's
//! device is not this driver's: its interrupt line is a GpioInt that
//! driver.gpio0 reports only to the claimant of the bus controller, so this
//! driver asks under its own key for the line the holder names.

use crate::facts::INPUT_CLOCK_HZ;
use crate::gpio0::{look, Doorbell};

/// `device u16, 0, 0, input_clock u32, key u64`.
pub fn opened(h: &crate::setup::Host) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[0..2].copy_from_slice(&h.device.to_le_bytes());
    b[4..8].copy_from_slice(&INPUT_CLOCK_HZ.to_le_bytes());
    b[8..16].copy_from_slice(&h.key.to_le_bytes());
    b
}

/// `present u32, rang u32`: present is 0 while gpio0 is absent or refuses.
pub fn doorbell(key: u64, uid: u16, pin: u16) -> [u8; 8] {
    let (present, rang) = match look(&Doorbell { key, uid, pin }) {
        Some(r) => (1u32, u32::from(r)),
        None => (0, 0),
    };
    let mut b = [0u8; 8];
    b[0..4].copy_from_slice(&present.to_le_bytes());
    b[4..8].copy_from_slice(&rang.to_le_bytes());
    b
}
