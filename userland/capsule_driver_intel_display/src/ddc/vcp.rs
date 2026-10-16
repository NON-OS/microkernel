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
//! gave; the input source, set only to an input the display's capabilities
//! list (it takes the picture away from this machine, so only the shell
//! asks it, on the person's word); the power mode, set only to on (1) or
//! off (4), and only as the capabilities list them. Any other code, or one
//! the capabilities do not name, is refused before the bus is touched.

use super::caps::Caps;
use super::error::DdcError;

pub const BRIGHTNESS: u8 = 0x10;
pub const CONTRAST: u8 = 0x12;
pub const INPUT_SOURCE: u8 = 0x60;
pub const POWER_MODE: u8 = 0xD6;

pub const ALLOWED: [u8; 4] = [BRIGHTNESS, CONTRAST, INPUT_SOURCE, POWER_MODE];

pub fn may_read(code: u8, caps: &Caps) -> Result<(), DdcError> {
    if !ALLOWED.contains(&code) {
        return Err(DdcError::NotAllowed);
    }
    caps.has(code).then_some(()).ok_or(DdcError::Unsupported)
}

/// Whether `value` may be written to `code`, given the most it reads and
/// what the display listed for it.
pub fn may_set(code: u8, value: u16, max: u16, caps: &Caps) -> Result<(), DdcError> {
    match code {
        BRIGHTNESS | CONTRAST if value <= max => Ok(()),
        POWER_MODE if (value == 1 || value == 4) && caps.lists(code, value) => Ok(()),
        INPUT_SOURCE if caps.lists(code, value) => Ok(()),
        BRIGHTNESS | CONTRAST | POWER_MODE | INPUT_SOURCE => Err(DdcError::Value),
        _ => Err(DdcError::NotAllowed),
    }
}
