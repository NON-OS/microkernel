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

use crate::error::PadError;

// Linux pinctrl-amd.h: each pin owns one dword at pin * 4 from the bank
// base, and bit 16 (PIN_STS_OFF) is the raw level on the pin.
pub const AMD_PIN_STS: u32 = 1 << 16;

/// Offset of `pin`'s register in a GPIO bank window of `window` bytes. Linux
/// amd_gpio_probe sizes the bank as resource_size / 4 pins.
pub fn amd_pin_reg(pin: u32, window: u64) -> Result<u64, PadError> {
    let off = u64::from(pin) * 4;
    if off + 4 > window {
        return Err(PadError::OutsideWindow);
    }
    Ok(off)
}

/// A pin register that reads all ones is a bank that does not decode; its
/// PIN_STS would read as a line held high for ever.
pub fn amd_alive(value: u32) -> Result<(), PadError> {
    if value == u32::MAX {
        return Err(PadError::Absent);
    }
    Ok(())
}
