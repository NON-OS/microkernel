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

//! The controls this driver will speak of, and nothing else (MCCS codes):
//! brightness and contrast, read and set within the maximum the monitor
//! gave; the input source, read only, since switching it would take the
//! picture away from this machine; the power mode, set only to on (1) or
//! off (4). Any other code is refused before the bus is touched.

use super::error::DdcError;

pub const BRIGHTNESS: u8 = 0x10;
pub const CONTRAST: u8 = 0x12;
pub const INPUT_SOURCE: u8 = 0x60;
pub const POWER_MODE: u8 = 0xD6;

pub const ALLOWED: [u8; 4] = [BRIGHTNESS, CONTRAST, INPUT_SOURCE, POWER_MODE];

pub fn may_read(code: u8) -> Result<(), DdcError> {
    ALLOWED.contains(&code).then_some(()).ok_or(DdcError::NotAllowed)
}

/// Whether `value` may be written to `code`, given the most it reads.
pub fn may_set(code: u8, value: u16, max: u16) -> Result<(), DdcError> {
    match code {
        BRIGHTNESS | CONTRAST if value <= max => Ok(()),
        BRIGHTNESS | CONTRAST => Err(DdcError::Value),
        POWER_MODE if value == 1 || value == 4 => Ok(()),
        POWER_MODE => Err(DdcError::Value),
        INPUT_SOURCE => Err(DdcError::ReadOnly),
        _ => Err(DdcError::NotAllowed),
    }
}
