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

// The AMD FCH GPIO bank (AMD's BIOS and Kernel Developer's Guides and
// Processor Programming References, GPIO section): one dword per pin at
// pin * 4 from the bank base, bit 16 the level on the pin. Recalled, not
// read: unverified.
pub const AMD_PIN_STS: u32 = 1 << 16;

/// Offset of `pin`'s register in a GPIO bank window of `window` bytes: a
/// pin whose dword does not lie inside the window is refused.
pub fn amd_pin_reg(pin: u32, window: u64) -> Result<u64, PadError> {
    let off = u64::from(pin) * 4;
    if off + 4 > window {
        return Err(PadError::OutsideWindow);
    }
    Ok(off)
}

/// A pin register that reads all ones is a bank that does not decode; its
/// level bit would read as a line held high for ever.
pub fn amd_alive(value: u32) -> Result<(), PadError> {
    if value == u32::MAX {
        return Err(PadError::Absent);
    }
    Ok(())
}
