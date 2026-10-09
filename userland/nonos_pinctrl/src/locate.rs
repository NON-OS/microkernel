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

use crate::controller::{controller, Controller};
use crate::error::PinError;
use crate::resolve::resolve;

/// Where a firmware pin's level lives, before any register is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    /// Pad `pad` of the community in window `bar`; its PADCFG0 offset needs
    /// that window's REVID and PADBAR (`intel_padcfg0`).
    Intel { bar: u8, pad: u16 },
    /// Pin `pin` of the single AMD bank window (`amd_pin_reg`).
    Amd { pin: u32 },
}

/// Locate GpioInt pin `pin` on the controller whose `_HID` is `hid`.
pub fn locate(hid: &[u8; 8], pin: u32) -> Result<Line, PinError> {
    match controller(hid).ok_or(PinError::UnknownController)? {
        Controller::Amd => Ok(Line::Amd { pin }),
        Controller::Intel(layout) => {
            let p = resolve(layout, pin)?;
            Ok(Line::Intel { bar: p.bar, pad: p.pad })
        }
    }
}
